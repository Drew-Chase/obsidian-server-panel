import {Switch} from "@nextui-org/react";
import {FontAwesomeIcon} from "@fortawesome/react-fontawesome";
import {faMoon, faSun} from "@fortawesome/free-solid-svg-icons";
import {applyTheme, getCurrentTheme, Theme} from "../ts/Theme.ts";

export default function ThemeSwitcher()
{
    return (
        <Switch
            size="lg"
            color="primary"
            startContent={<FontAwesomeIcon icon={faSun}/>}
            endContent={<FontAwesomeIcon icon={faMoon}/>}
            defaultSelected={getCurrentTheme() === Theme.light}
            onValueChange={
                (value) =>
                {
                    applyTheme(value ? Theme.light : Theme.dark);
                }
            }
        />
    );
}
