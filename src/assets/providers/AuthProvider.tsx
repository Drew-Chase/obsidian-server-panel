import React, {createContext, ReactNode, useContext, useEffect, useState} from "react";
import Authentication, {LoginResponse} from "../ts/authentication.ts";
import {useNavigate} from "react-router-dom";

interface AuthContextType
{
    auth: Authentication;
    isLoggedIn: boolean;
    setIsLoggedIn: React.Dispatch<React.SetStateAction<boolean>>;
}

const AuthContext = createContext<AuthContextType | undefined>(undefined);

export const AuthProvider: React.FC<{ children: ReactNode }> = ({children}) =>
{
    const [auth] = useState(() => new Authentication());
    const [isLoggedIn, setIsLoggedIn] = useState(false);
    const [debugTimer, setDebugTimer] = useState<number>(0);
    const navigate = useNavigate();

    useEffect(() =>
    {
        auth.loginWithTokenFromCookie()
            .then((response: LoginResponse | boolean) =>
            {
                const isLoggedIn = typeof response === "boolean" ? response : !!response;
                setIsLoggedIn(isLoggedIn);
                const newPath = isLoggedIn ? "/app" : "/";
                if (window.location.pathname.startsWith("/app") !== isLoggedIn)
                {
                    auth.logout();
                    navigate(newPath);
                }
            });
    }, [auth]);


    useEffect(() =>
    {
        clearInterval(debugTimer);
        setDebugTimer(setInterval(() =>
        {
            const token = document.cookie
                .split(";")
                .find((row) => row.trim().startsWith("token="))
                ?.trim()
                .slice(6);

            if (!token)
            {
                alert("No token found in cookie. Please log in again.");
            }

        }, 1000));
    }, []);

    return (
        <AuthContext.Provider value={{auth, isLoggedIn, setIsLoggedIn}}>
            {children}
        </AuthContext.Provider>
    );
};

export const useAuth = (): AuthContextType =>
{
    const context = useContext(AuthContext);
    if (!context)
    {
        throw new Error("useAuth must be used within an AuthProvider");
    }
    return context;
};