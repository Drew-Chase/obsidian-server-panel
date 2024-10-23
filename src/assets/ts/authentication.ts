// noinspection ExceptionCaughtLocallyJS

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

        } catch (e)
        {
            this.token = null;
        }
    }

    /**
     * Authenticates a user with the provided username and password.
     *
     * @param {string} username - The username of the user attempting to log in.
     * @param {string} password - The password of the user attempting to log in.
     * @param {boolean} [rememberMe=false] - A flag indicating whether the user should be remembered for future sessions.
     * @return {Promise<LoginResponse|ErrorResponse>} A Promise that resolves to a LoginResponse if authentication is successful, or an ErrorResponse if it fails.
     */
    public async login(username: string, password: string, rememberMe: boolean = false): Promise<LoginResponse | ErrorResponse>
    {

        let response: any, data: LoginResponse | ErrorResponse;
        try
        {
            response = await fetch(`/api/auth/login`, {
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
            this.generateCookies(data.token, rememberMe);
            return data;
        } else
        {
            return data;
        }
    }

    /**
     * Logs in a user using a provided token.
     *
     * @param {string} token - The token used for authentication.
     * @return {Promise<LoginResponse>} A promise that resolves to the login response.
     */
    public async loginWithToken(token: string): Promise<LoginResponse>
    {

        try
        {
            const response = await fetch(`/api/auth/login/token`, {
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
            if (!response.ok)
                throw new Error(JSON.stringify(data));

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
            const response = await fetch(``, {method: "POST", body: formData});
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
     * Logs in a user using the token stored in the cookie.
     * This method checks if a token is present and, if so, attempts to log in using that token.
     *
     * @return {Promise<LoginResponse | boolean>} A promise that resolves to a LoginResponse if the login is successful, or false if no token is available.
     */
    public async loginWithTokenFromCookie(): Promise<LoginResponse | boolean>
    {
        return this.token === null ? false : await this.loginWithToken(this.token);
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
     * Generates a cookie with the provided token.
     *
     * @param {string} token - The token to be stored in the cookie.
     * @param {boolean} [rememberMe=false] - Whether the cookie should have a long expiration date for remembering the user.
     * @return {void}
     */
    public generateCookies(token: string, rememberMe: boolean = false): void
    {
        if (!rememberMe)
        {
            document.cookie = `token=${token}; path=/; domain=.${window.location.hostname}; samesite=strict`;
        } else
        {
            let expire = new Date();
            expire.setFullYear(3000, 0, 1);
            console.log(`token=${token}; path=/; domain=.${window.location.hostname}; samesite=strict; expires=${expire.toUTCString()}`);
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
