interface MagnifyGlassIconProperties
{
    width?: number | string;
    height?: number | string;
    size?: number | string;
    fill?: string;
}

export default function MagnifyGlass(props: MagnifyGlassIconProperties)
{
    return (
        <svg width={props.size || props.width || "15"} height={props.size || props.height || "15"} viewBox="0 0 15 15" fill={"none"} xmlns="http://www.w3.org/2000/svg">
            <path d="M7.22917 12.375C10.0711 12.375 12.375 10.0711 12.375 7.22917C12.375 4.3872 10.0711 2.08334 7.22917 2.08334C4.3872 2.08334 2.08333 4.3872 2.08333 7.22917C2.08333 10.0711 4.3872 12.375 7.22917 12.375Z" stroke={props.fill || "currentColor"} strokeWidth="2.5" strokeLinecap="round" strokeLinejoin="round"/>
            <path d="M12.9167 12.9167L11.8333 11.8333" stroke={props.fill || "currentColor"} strokeWidth="2.5" strokeLinecap="round" strokeLinejoin="round"/>
        </svg>

    );
}