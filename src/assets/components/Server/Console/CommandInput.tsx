import OInput from "../../Extends/OInput.tsx";
import {FontAwesomeIcon} from "@fortawesome/react-fontawesome";
import {faPaperPlane, faTerminal} from "@fortawesome/free-solid-svg-icons";
import {Button} from "@nextui-org/react";
import {useState} from "react";
import {useSelectedServer} from "../../../providers/SelectedServerProvider.tsx";
import $ from "jquery";

export default function CommandInput()
{
    const {server} = useSelectedServer();
    const [isLoading, setIsLoading] = useState(false);
    const sendCommand = async () =>
    {
        let input = $("#command-input");
        const command = input.val() as string;
        if (command?.trim() === "") return;

        setIsLoading(true);
        await server?.sendCommand(command);
        input.val("");
        setIsLoading(false);

        input.trigger("focus");
    };
    return (
        <OInput
            id={"command-input"}
            label={"Command"}
            placeholder={"Enter command..."}
            startContent={<FontAwesomeIcon icon={faTerminal}/>}
            endContent={<Button variant={"light"} onClick={sendCommand} isLoading={isLoading}>{!isLoading && <FontAwesomeIcon icon={faPaperPlane}/>}</Button>}
            className={"w-full drop-shadow-lg shrink-0 pr-0"}
            isReadOnly={isLoading}
            onKeyUp={async e =>
            {
                if (e.key === "Enter")
                {
                    e.preventDefault();
                    await sendCommand();
                }
            }}
        />
    );
}