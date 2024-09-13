import $ from "jquery";

/**
 * Represents the theme options for a user interface.
 * The available themes are `default`, `light`, and `dark`.
 *
 * @enum {string}
 * @readonly
 * @property {string} default - The default theme.
 * @property {string} light - The light theme.
 * @property {string} dark - The dark theme.
 */
export enum Theme
{
    default,
    light,
    dark
}

/**
 * Applies the specified theme to the application.
 *
 * @param {Theme} [theme=Theme.default] - The theme to apply. Defaults to the default theme.
 *
 * @return {void}
 */
export function applyTheme(theme: Theme = Theme.default): void
{
    const name: string = theme == Theme.light ? "light" : theme == Theme.dark ? "dark" : (localStorage.getItem("theme") ?? "light");
    localStorage.setItem("theme", name);
    $("html").removeClass("dark").removeClass("light").addClass(name);
}

/**
 * Retrieves the current theme value from the local storage and returns the corresponding theme.
 *
 * @return {Theme} The current theme value:
 *                 - Theme.dark if the theme value in local storage is "dark".
 *                 - Theme.light if the theme value in local storage is "light" or no theme value is found.
 */
export function getCurrentTheme(): Theme
{
    switch (localStorage.getItem("theme"))
    {
        case "dark":
            return Theme.dark;
        case "light":
        default:
            return Theme.light;
    }
}