import { useTranslation } from "react-i18next";
import { invoke } from "@tauri-apps/api/core";
import { openUrl } from "@tauri-apps/plugin-opener";
import { useNavigate } from "react-router";
import { FormEvent, useEffect } from "react";
import { store, setUserData } from "../../shared/libraries/storeUtilis";
import "./index.css"

// FIX недо дорабтать сохранение токена в сессию если поользователь не сохранил токен через checkBox. Да в принципи код улучшить слишком кривоой useEffect
function Authentication() {
    const { t } = useTranslation();
    const navigate = useNavigate()

    useEffect(() => {
        const tryAutoLogin = async () => {
            const accessToken = await store.get('accessToken');
            if (accessToken) {
                try {
                    const response: ApiResponse<AuthenticationResponse> = await invoke("check_token", { accessToken });
                    if (response.data?.accessToken) {
                        setUserData(response);
                        navigate("/home");
                    }
                } catch (err) {
                    console.error(err)
                }
            }
        };
        tryAutoLogin();
    }, [navigate]);

    const submit = async (e: FormEvent<HTMLFormElement>) => {
        e.preventDefault();
        const formData = new FormData(e.currentTarget);
        const login = formData.get("login") as string;
        const password = formData.get("password") as string;
        const autoLogin = formData.get("setAutoAuthentication") === "on";
        if (login.length < 3) return;
        try {
            const response: ApiResponse<AuthenticationResponse> = await invoke("authentication", { login, password });
            if (response.data?.accessToken && autoLogin) {
                await store.set('accessToken', response.data.accessToken);
                await store.save();
            }
            setUserData(response);
            navigate("/home");
        } catch (err) {
            console.error(err);
        }
    }
    

    return(
        <div className="authenticationPages">
            <div className="authenticationBlock">
                <form className="authenticationForm" onSubmit={submit}>
                    <img className="logo" src="tauri.svg" alt="" />
                    <input type="text" name="login" placeholder={t("authentication.form.login")} />
                    <input type="password" name="password" placeholder={t("authentication.form.password")}/>
                    <button className="enterButton">{t("authentication.form.enter")}</button>
                    <label className="autoAuthentication">
                        <input 
                            type="checkbox"
                            name="setAutoAuthentication"
                            defaultChecked={true}
                         />
                        <span>
                            <svg viewBox="0 0 20 20">
                                <path d="M16.707 5.293a1 1 0 010 1.414l-8 8a1 1 0 01-1.414 0l-4-4a1 1 0 011.414-1.414L8 12.586l7.293-7.293a1 1 0 011.414 0z" clipRule="evenodd"/>
                            </svg>
                        </span>
                        Автоматический вход
                    </label>
                    <a onClick={() => openUrl("https://arsland.ru")}>{t("authentication.form.ref_register_account")}</a>
                </form>
            </div>
        </div>
    )
}

export default Authentication;