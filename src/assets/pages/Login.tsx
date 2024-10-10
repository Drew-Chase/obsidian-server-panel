import {Button, Card, CardBody, CardHeader, Input} from "@nextui-org/react";
import {useState} from "react";
import {FontAwesomeIcon} from "@fortawesome/react-fontawesome";
import {faEnvelope, faEye, faEyeSlash, faKey} from "@fortawesome/free-solid-svg-icons";
import {useNavigate} from "react-router-dom";
import {setTitle} from "../../main.tsx";
import ExtendedSwitch from "../components/Extends/ExtendedSwitch.tsx";
import {useAuth} from "../providers/AuthProvider.tsx";
import {ErrorResponse, LoginResponse} from "../ts/authentication.ts";

export default function Login()
{
    setTitle("Login");

    const [showPassword, setShowPassword] = useState(false);
    const [isLoggingIn, setIsLoggingIn] = useState(false);
    const [rememberMe, setRememberMe] = useState(false);

    const [username, setUsername] = useState("");
    const [password, setPassword] = useState("");

    const [usernameError, setUsernameError] = useState("");
    const [passwordError, setPasswordError] = useState("");
    const [error, setError] = useState("");
    const {auth, setIsLoggedIn, isLoggedIn} = useAuth();
    const navigate = useNavigate();

    if (isLoggedIn)
    {
        // navigate("/app/");
    }


    const resetErrors = () =>
    {
        setUsernameError("");
        setPasswordError("");
        setError("");
    };

    const login = async () =>
    {
        if (isLoggingIn) return;
        resetErrors();
        if (!username || !password)
        {
            if (!username) setUsernameError("Username is required");
            if (!password) setPasswordError("Password is required");
            return;
        }
        setIsLoggingIn(true);
        const expiration = rememberMe ? 60 * 60 * 24 * 365 * 1000 : -1; // 1000 years in the future, this is a hacky way to keep the user logged in
        const response: LoginResponse | ErrorResponse = await auth.login(username, password, expiration);

        console.log("Login response", response);
        if ("token" in response) // If the response is a LoginResponse
        {
            if ((response as LoginResponse).token)
            {
                setIsLoggedIn(true);
                navigate("/app/");
            } else
            {
                console.error("No token was provided in message!");
                setError("No token was provided in message!");
            }
        } else
        {
            if ("error" in response)
            {
                let message = (response as ErrorResponse).error;
                if (message.toLowerCase().includes("password"))
                {
                    setPasswordError(`Invalid Password`);
                }
                if (message.toLowerCase().includes("user"))
                {
                    setUsernameError("Invalid Username or Email");
                    setPasswordError("Invalid Password");
                }
                console.error("Error message", message);
                setError(message);
            } else
            {
                setError("An unknown error occurred.");
            }
        }

        setIsLoggingIn(false);
    };

    return (
        <>
            <div className={"h-[calc(100dvh_-_12rem)]"}>

                <Card
                    className={"flex flex-col w-1/3 max-w-[800px] min-w-[400px] mx-auto mt-[100px] justify-center px-8 py-4 bg-custom-gradient"}
                >
                    <CardHeader><h1 className={"text-5xl"}>Login</h1></CardHeader>
                    <CardBody>
                        <div className={"flex flex-col gap-4"}>
                            <Input
                                autoFocus
                                label={"Email or Username"}
                                placeholder={"Enter your username or email"}
                                type={"text"}
                                variant={"underlined"}
                                startContent={<FontAwesomeIcon icon={faEnvelope} opacity={.5}/>}
                                autoComplete={"username webauthn"}
                                value={username}
                                onValueChange={setUsername}
                                isInvalid={!!usernameError}
                                errorMessage={usernameError}
                                onKeyUp={(e) => e.key === "Enter" && login()}
                            />
                            <Input
                                label={"Password"}
                                variant={"underlined"}
                                placeholder={"Enter your password"}
                                type={showPassword ? "text" : "password"}
                                autoComplete={"current-password"}
                                startContent={<FontAwesomeIcon icon={faKey} opacity={.5}/>}
                                endContent={
                                    <FontAwesomeIcon
                                        onClick={() => setShowPassword(prev => !prev)}
                                        icon={showPassword ? faEye : faEyeSlash}
                                        opacity={showPassword ? 1 : 0.5}
                                        className={"cursor-pointer"}
                                    />
                                }
                                value={password}
                                onValueChange={setPassword}
                                isInvalid={!!passwordError}
                                errorMessage={passwordError}
                                onKeyUp={(e) => e.key === "Enter" && login()}
                            />
                            <ExtendedSwitch
                                label={"Remember Me?"}
                                description={"This will keep you logged in until you log out."}
                                className={"max-w-full"}
                                onToggle={setRememberMe}
                                toggle={rememberMe}
                            />
                            {error && <p className={"text-danger"}><strong>Error:</strong> {error}</p>}
                            <Button radius={"lg"} color={"primary"} isLoading={isLoggingIn} onClick={login}>Log In</Button>
                        </div>
                    </CardBody>

                </Card>

            </div>
        </>
    );
}
