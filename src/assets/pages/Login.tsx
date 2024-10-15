import {Button, Card, CardBody, CardHeader, cn, Input, Link, Image} from "@nextui-org/react";
import {useState} from "react";
import {FontAwesomeIcon} from "@fortawesome/react-fontawesome";
import {faEnvelope, faEye, faEyeSlash, faKey} from "@fortawesome/free-solid-svg-icons";
import {useNavigate} from "react-router-dom";
import {setTitle} from "../../main.tsx";
import ExtendedSwitch from "../components/Extends/ExtendedSwitch.tsx";
import {useAuth} from "../providers/AuthProvider.tsx";
import {ErrorResponse, LoginResponse} from "../ts/authentication.ts";
import {Navbar, NavbarContent, NavbarItem, NavbarBrand} from "@nextui-org/navbar";
import Logo from "../images/logo.gif";
import {faGithub} from "@fortawesome/free-brands-svg-icons";
import BackgroundImage from "../images/mc-bg.webp";

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
        navigate("/app/");
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
        <div className={"rounded-xl p-2 overflow-hidden relative"}>
            <div
                className={"absolute top-[-10px] bottom-[-10px] left-[-10px] right-[-10px] bg-cover blur-md"}
                style={{
                    backgroundImage: `linear-gradient(rgba(45, 55, 72, 0.47) 0%, rgb(26, 32, 46) 100%), url(${BackgroundImage})`
                }}
            >
            </div>

            <Navbar maxWidth={"full"} className={"rounded-xl"}>
                <NavbarBrand className={"gap-4"}>
                    <Image src={Logo} width={32} radius={"sm"}/>
                    <h1 className={"font-semibold text-[1.25rem]"}>Obsidian</h1>
                </NavbarBrand>
                <NavbarContent justify={"end"}>
                    <NavbarItem
                        as={Link}
                        href={"https://github.com/drew-chase/obsidian-server-panel"}
                        className={"h-full text-inherit opacity-50 data-[hover=true]:opacity-100"}
                        target={"_blank"}
                    >
                        <FontAwesomeIcon
                            icon={faGithub}
                            width={24}
                            className={"h-full"}
                        />
                    </NavbarItem>
                </NavbarContent>
            </Navbar>
            <div className={"flex flex-col h-[calc(100dvh_-_11.7rem)] min-h-[300px] pb-3 my-[10px]"}>
                <Card
                    className={"flex flex-col w-1/3 max-w-[900px] min-w-[400px] min-h-[200px] m-auto justify-center px-8 py-4 backdrop-blur-lg backdrop-saturate-150 bg-background/70"}
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

            <footer
                className={
                    cn(
                        "flex items-center justify-center inset-x-0 rounded-xl z-40 w-full h-auto p-2",
                        "backdrop-blur-lg backdrop-saturate-150 bg-background/70"
                    )
                }
            >
                <p>Created by Drew Chase</p>
            </footer>
        </div>
    );
}
