interface ForgeIconIconProperties
{
    width?: number | string;
    height?: number | string;
    size?: number | string;
    fill?: string;
}

export default function ForgeIcon(props: ForgeIconIconProperties)
{
    return (
        <svg xmlns="http://www.w3.org/2000/svg" version="1.1" x="0px" y="0px" viewBox="0 0 100 100" enableBackground="new 0 0 100 100" width={props.size || props.width || 100} height={props.size || props.height || 100}>
            <path d="M5,30.059h0.004c0-0.001,0-0.002,0-0.003L5,30.059z M95,35.759v-5.7l0,0v-3.943H32.149v3.943H5.004  c0.003,8.306,10.956,15.039,24.465,15.039c7.68,0,13.905,2.961,13.905,6.613l0.001-0.001c0,4.482-3.755,8.528-9.798,11.421  c-2.53,1.211-4.102,3.812-4.102,6.617v4.137h14.75c0-1.888,4.076-3.412,9.101-3.412c5.024,0,9.099,1.524,9.099,3.412h14.764v-3.801  c0-2.849-1.616-5.492-4.211-6.672c-6.395-2.91-10.4-7.074-10.4-11.702C62.576,42.896,77.088,35.759,95,35.759z"
                  fill={props.fill || "currentColor"}/>
        </svg>
    );
}