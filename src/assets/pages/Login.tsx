import {Button, Card, CardBody, CardHeader, Input} from "@nextui-org/react";
import {useState} from "react";
import {FontAwesomeIcon} from "@fortawesome/react-fontawesome";
import {faEnvelope, faEye, faEyeSlash, faKey} from "@fortawesome/free-solid-svg-icons";
import Authentication from "../ts/Authentication.ts";
import {useNavigate} from "react-router-dom";
import {debug_mode, setTitle} from "../../main.tsx";
import ExtendedSwitch from "../components/Extends/ExtendedSwitch.tsx";

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
    const navigate = useNavigate();

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
        if (debug_mode)
        {
            console.log("Logging in", {username, password, rememberMe});
            navigate("/app/");
            setIsLoggingIn(false);
            return;

        }
        const response = await Authentication.getInstance().login(username, password, rememberMe);
        if (typeof response === "string")
        {
            setUsernameError("Invalid username");
            setPasswordError("Invalid password");
            setError(response);
        }
        if (typeof response === "object" && response.token) navigate("/app/");

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
                                selected={rememberMe}
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
