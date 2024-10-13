import {api_domain} from "../../main.tsx";

export interface UserProfile
{
    username: string;
    admin: boolean;
    token: string;
}

export interface LoginResponse
{
    id: number;
    username: string;
    admin: boolean;
    created_at: string;
    updated_at: string;
    last_login: string;
    token: string;
}

export interface ErrorResponse
{
    error: string;
}

export interface RegistrationResponse
{
    success: boolean;
    message: string;
    user: string;
}

/**
 * Represents a class for authentication.
 * @class
 */
export default class Authentication
{
    token: string | null;

    constructor()
    {
        try
        {
            this.token = document.cookie
                .split(";")
                .find((row) => row.trim().startsWith("token="))
                ?.trim()
                .slice(6) ?? null;

            this.loginWithTokenFromCookie().then((response) =>
            {
                console.log(response);
            });

        } catch (e)
        {
            this.token = null;
        }
    }

    /**
     * Login method for authenticating a user.
     *
     * @param {string} username - The username of the user.
     * @param {string} password - The password of the user.
     * @param {number} [expiration=-1] - The expiration time of the generated token.
     * @return {Promise<JSON>} - A Promise that resolves to a JSON object containing the login response data.
     * @throws {Error} - Throws an Error object if an error occurs during the login process.
     */
    public async login(username: string, password: string, expiration: number = -1): Promise<LoginResponse | ErrorResponse>
    {

        let response: any, data: LoginResponse | ErrorResponse;
        try
        {
            response = await fetch(`${api_domain}/auth/login`, {
                method: "POST",
                body: JSON.stringify({username, password}),
                headers: {
                    "Content-Type": "application/json"
                }
            });

            data = await response.json();
        } catch (err)
        {
            throw err;
        }
        if ("token" in data)
        {
            this.generateCookies(data.token, expiration);
            return data;
        } else
        {
            return data;
        }
    }

    /**
     * Logs in a user with a token.
     *
     * @param {string} token - The token to be used for authentication.
     * @param {number} [expiration=-1] - The expiration time for the generated cookies.
     * @returns {Promise<JSON>} - A Promise that resolves to the response data in JSON format.
     * @throws {Error} - Throws an error if the login process fails.
     */
    public async loginWithToken(token: string, expiration: number = -1): Promise<LoginResponse>
    {

        try
        {
            const response = await fetch(`${api_domain}/auth/login/token`, {
                method: "POST",
                body: JSON.stringify({token}),
                headers: {
                    "Content-Type": "application/json"
                }
            });

            const data = await response.json();

            if ("error" in data)
            {
                throw new Error(JSON.stringify(data));
            }
            if (response.ok)
            {
                if (data)
                {
                    this.generateCookies(token, expiration);
                }
            } else
            {
                throw new Error(JSON.stringify(data));
            }

            return data;
        } catch (error)
        {
            console.error("Unable to login with token:", error);
            throw error;
        }
    }

    /**
     * Registers a user with the specified username and password.
     *
     * @param {string} username - The username of the user to register.
     * @param {string} password - The password of the user to register.
     * @returns {Promise<Object|undefined>} - A promise that resolves to the registration response data
     * if successful, or rejects with an error if registration fails.
     */
    public async register(username: string, password: string): Promise<RegistrationResponse | undefined>
    {
        const formData = new FormData();
        formData.append("username", username);
        formData.append("password", password);

        try
        {
            const response = await fetch(`${api_domain}`, {method: "POST", body: formData});
            const data = await response.json();
            if (response.ok)
            {
                if (data.success)
                {
                    return data;
                }
            } else
            {
                if (!data.message)
                {
                    data.message = "An unknown error occurred.";
                }
                throw new Error(JSON.stringify(data));
            }
        } catch (error)
        {
            console.error(error);
            throw error;
        }

    }

    /**
     * Logs in the user with the token obtained from the cookie.
     *
     * @param {number} expiration - The expiration time of the token in minutes. Default is -1 (no expiration).
     * @return {Promise<JSON | boolean>} - A promise that resolves with JSON data if the login is successful, false otherwise.
     */
    public async loginWithTokenFromCookie(expiration: number = -1): Promise<LoginResponse | boolean>
    {
        return this.token === null ? false : await this.loginWithToken(this.token, expiration);
    }

    /**
     * Logout the user and clear the token cookie.
     *
     * @return {void}
     */
    public logout(): void
    {
        document.cookie = `token=; path=/; domain=.${window.location.hostname}; samesite=strict; expires=Thu, 01 Jan 1970 00:00:00 GMT`;
    }

    /**
     * Generates cookies for the given token.
     *
     * @param {string} token - The token used for generating the cookies.
     * @param {number} [days=-1] - The number of days the cookie should be valid for. Default value is -1.
     *
     * @returns {void}
     */
    public generateCookies(token: string, days: number = -1): void
    {
        if (days <= 0)
        {
            document.cookie = `token=${token}; path=/; domain=.${window.location.hostname}; samesite=strict`;
        } else
        {
            let expire = new Date();
            expire.setDate(expire.getDate() + days);
            document.cookie = `token=${token}; path=/; domain=.${window.location.hostname}; samesite=strict; expires=${expire.toUTCString()}`;
        }
        this.token = token;
    }

    /**
     * Retrieves the user profile from the logged in user's token.
     *
     * @throws {Error} If the user is not logged in or if the token is invalid.
     * @returns {UserProfile} The user profile parsed from the token.
     */
    public getUserProfile(): UserProfile
    {
        if (!this.token) throw new Error("User is not logged in");
        return JSON.parse(atob(this.token!));
    }
}
