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
        <svg width={props.size || props.width || "16"} height={props.size || props.height || "16"} viewBox="0 0 16 16" fill={"none"} xmlns="http://www.w3.org/2000/svg">
            <path d="M7.25929 12.9445C10.5321 12.9445 13.1852 10.2914 13.1852 7.01861C13.1852 3.74581 10.5321 1.09268 7.25929 1.09268C3.98649 1.09268 1.33336 3.74581 1.33336 7.01861C1.33336 10.2914 3.98649 12.9445 7.25929 12.9445Z" stroke="currentColor" strokeLinecap="round" strokeLinejoin="round"/>
            <path d="M14.6666 14.426L11.4444 11.2038" stroke="currentColor" stroke-linecap="round" stroke-linejoin="round"/>
        </svg>

    );
}