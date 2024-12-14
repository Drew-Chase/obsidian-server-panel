import React, {createContext, ReactNode, useContext, useEffect, useState} from "react";
import Authentication, {LoginResponse} from "../ts/authentication.ts";
import {useLocation, useNavigate} from "react-router-dom";

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
    const navigate = useNavigate();
    const {pathname} = useLocation();

    useEffect(() =>
    {
        auth.loginWithTokenFromCookie()
            .then((response: LoginResponse | boolean) =>
            {
                const isLoggedIn = typeof response === "boolean" ? response : !!response;
                setIsLoggedIn(isLoggedIn);
                if (!isLoggedIn && pathname.startsWith("/app"))
                {
                    navigate("/");
                    auth.logout();
                } else
                {
                    if (!pathname.startsWith("/app"))
                        navigate("/app");
                }
            });
    }, [auth]);

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