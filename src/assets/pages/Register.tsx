import {setTitle} from "../../main.tsx";
import {useState} from "react";
import {useNavigate, useSearchParams} from "react-router-dom";
import {Button, Card, CardBody, CardHeader, Input, Link} from "@nextui-org/react";
import {FontAwesomeIcon} from "@fortawesome/react-fontawesome";
import {faEnvelope, faEye, faEyeSlash, faKey} from "@fortawesome/free-solid-svg-icons";

export default function Register()
{
    setTitle("Register");

    const [showPassword, setShowPassword] = useState(false);
    const [showConfirmPassword, setShowConfirmPassword] = useState(false);
    const [isLoggingIn, setIsLoggingIn] = useState(false);

    const [username, setUsername] = useState("");
    const [password, setPassword] = useState("");
    const [confirmPassword, setConfirmPassword] = useState("");

    const [usernameError, setUsernameError] = useState("");
    const [passwordError, setPasswordError] = useState("");
    const [error, setError] = useState("");

    const [params, _] = useSearchParams();
    const code = params.get("code");
    // @ts-ignore
    const navigate = useNavigate();

    console.log("Code", code);

    const resetErrors = () =>
    {
        setUsernameError("");
        setPasswordError("");
        setError("");
    };

    const register = async () =>
    {
        setIsLoggingIn(true);
        resetErrors();

        if (!username || !password || password !== confirmPassword)
        {
            setUsernameError(!username ? "Username is required" : "");
            setPasswordError(!password ? "Password is required" : password !== confirmPassword ? "Passwords do not match" : "");
            setIsLoggingIn(false);
            return;
        }

        setIsLoggingIn(false);
    };


    return (
        <>
            <div className={"h-[calc(100dvh_-_12rem)]"}>

                <Card
                    className={"flex flex-col w-1/3 max-w-[800px] min-w-[400px] mx-auto mt-[100px] justify-center px-8 py-4 bg-custom-gradient"}
                >
                    <CardHeader><h1 className={"text-5xl"}>Register</h1></CardHeader>
                    <CardBody>

                        {!code ? (
                            <>
                                <p className={"text-lg text-danger italic mb-4"}>No registration code was provided, please contact your system administrator for more information.</p>
                                <Button as={Link} href={"/"} radius={"lg"} color={"primary"}>Back to Login</Button>
                            </>
                        ) : (
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
                                    onKeyUp={(e) => e.key === "Enter" && register()}
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
                                    onKeyUp={(e) => e.key === "Enter" && register()}
                                />
                                <Input
                                    label={"Confirm Password"}
                                    variant={"underlined"}
                                    placeholder={"Confirm your password"}
                                    type={showConfirmPassword ? "text" : "password"}
                                    autoComplete={"current-password"}
                                    startContent={<FontAwesomeIcon icon={faKey} opacity={.5}/>}
                                    endContent={
                                        <FontAwesomeIcon
                                            onClick={() => setShowConfirmPassword(prev => !prev)}
                                            icon={showConfirmPassword ? faEye : faEyeSlash}
                                            opacity={showConfirmPassword ? 1 : 0.5}
                                            className={"cursor-pointer"}
                                        />
                                    }
                                    value={confirmPassword}
                                    onValueChange={setConfirmPassword}
                                    isInvalid={!!passwordError}
                                    errorMessage={passwordError}
                                    onKeyUp={(e) => e.key === "Enter" && register()}
                                />
                                {error && <p className={"text-danger"}><strong>Error:</strong> {error}</p>}
                                <Button radius={"lg"} color={"primary"} isLoading={isLoggingIn} onClick={register}>Register</Button>
                            </div>
                        )}
                    </CardBody>

                </Card>

            </div>
        </>
    );
}
