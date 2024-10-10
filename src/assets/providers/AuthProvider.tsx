import React, {createContext, ReactNode, useContext, useEffect, useState} from "react";
import Authentication, {LoginResponse} from "../ts/authentication.ts";

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
    const [isLoggedIn, setIsLoggedIn] = useState(auth.isLoggedIn);

    useEffect(() =>
    {
        auth.loginWithTokenFromCookie()
            .then((response: LoginResponse | boolean) =>
            {
                if (typeof response === "boolean")
                {
                    setIsLoggedIn(response);
                } else
                {
                    if (response) setIsLoggedIn(true);
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