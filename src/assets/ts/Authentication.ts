import $ from "jquery";
import {api_domain} from "../../main.tsx";

export interface User
{
    id: number;
    username: string;
    password: string;
    admin: boolean;
    created_at: string;
    updated_at: string;
    last_login: string;
}

export interface UserLogin
{
    username: string;
    password: string;
}

export interface UserRegistration
{
    username: string;
    password: string;
    access_token: string;
}

export interface UserResponse
{
    id: number;
    username: string;
    admin: boolean;
    created_at: string;
    updated_at: string;
    last_login: string;
    token: string;
}

export interface Token
{
    username: string;
    hash: string;
}

export default class Authentication
{
    private static _instance: Authentication;
    private _token: string;
    private _user: UserResponse;
    private _is_logged_in: boolean;

    private constructor()
    {
        this._token = "";
        this._user = {} as UserResponse;
        this._is_logged_in = false;
    }

    public async login(username: string, password: string, remember_me: boolean): Promise<UserResponse | string>
    {
        console.log("Logging in", {username, password, remember_me});
        try
        {
            let response: UserResponse = await $.ajax(`${api_domain}/api/auth/login`, {method: "POST", data: JSON.stringify({username: username, password: password}), contentType: "application/json", dataType: "json"});
            this._user = response;
            this._token = response.token;
            document.cookie = `token=${response.token}; path=/; ${remember_me ? "max-age=31536000" : ""}`;
            console.log("Log in successful", response);
            this._is_logged_in = true;
            localStorage.setItem("user", JSON.stringify(response));
            return response;
        } catch (e: any)
        {
            console.error("Failed to login", e);
            return e?.responseJSON?.message || "Invalid username or password";
        }
    }

    public async login_with_cookies(): Promise<UserResponse>
    {
        console.log("Logging in with cookies");
        let token: string = document.cookie.split("; ").find(row => row.startsWith("token="))?.substring(6) || "";
        if (token)
        {
            this._is_logged_in = true;
            return await this.login_with_token(token);
        }
        console.error("No token found in cookies");
        return {} as UserResponse;
    }

    public async login_with_token(token: string): Promise<UserResponse>
    {
        console.log("Logging in with token");
        try
        {
            let response: UserResponse = await $.ajax(`${api_domain}/api/auth/login/token`, {method: "POST", data: JSON.stringify({token: token}), contentType: "application/json", dataType: "json"}) as UserResponse;
            this._user = response;
            this._token = response.token;
            this._is_logged_in = true;
            localStorage.setItem("user", JSON.stringify(response));
            return response;
        } catch (e)
        {
            console.error("Failed to login with token", token, e);
            return {} as UserResponse;
        }
    }

    public logout(): void
    {
        console.log("Logging out");
        this._token = "";
        this._user = {} as UserResponse;
        document.cookie = `token=; path=/; max-age=0`;
        this._is_logged_in = false;
        localStorage.removeItem("user");
    }

    public static getInstance(): Authentication
    {
        if (!this._instance)
        {
            this._instance = new Authentication();
        }
        return this._instance;
    }

    public get token(): string
    {
        return this._token;
    }

    public get user(): UserResponse
    {
        return localStorage.getItem("user") ? JSON.parse(localStorage.getItem("user")!) : this._user;
    }

    public get is_logged_in(): boolean
    {
        return this._is_logged_in;
    }
}