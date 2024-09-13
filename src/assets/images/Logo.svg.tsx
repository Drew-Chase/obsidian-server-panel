interface LogoIconProperties
{
    width?: number | string;
    height?: number | string;
    size?: number | string;
}

export default function Logo(props: LogoIconProperties)
{
    return (
        <svg width={props.size || props.width || "27"} height={props.size || props.height || "27"} viewBox="0 0 27 27" fill="none" xmlns="http://www.w3.org/2000/svg">
            <path fill-rule="evenodd" clip-rule="evenodd"
                  d="M10.0536 0.275879C9.59481 0.275879 9.22287 0.647826 9.22287 1.10665V8.99895H9.22283C4.40521 8.99895 0.499756 12.9044 0.499756 17.722C0.499756 22.5396 4.40521 26.4451 9.22283 26.4451H17.1151C17.574 26.4451 17.9459 26.0731 17.9459 25.6143V17.722H17.9459C22.7636 17.722 26.669 13.8166 26.669 8.99895C26.669 4.18133 22.7636 0.275879 17.9459 0.275879H10.0536ZM17.9459 17.722V9.82971C17.9459 9.37089 17.574 8.99895 17.1151 8.99895H9.22287V16.8913C9.22287 17.3501 9.59481 17.722 10.0536 17.722H17.9459Z"
                  fill={"#CB3CFF"}/>
            <path d="M9.96003 0.275879C9.5529 0.275879 9.22287 0.605917 9.22287 1.01304L9.22287 8.99895H17.2088C17.6159 8.99895 17.9459 9.32899 17.9459 9.73611L17.9459 17.722C22.7636 17.722 26.669 13.8166 26.669 8.99895C26.669 4.18133 22.7636 0.275879 17.9459 0.275879H9.96003Z" fill={"#00C2FF"}/>
        </svg>

    );
}