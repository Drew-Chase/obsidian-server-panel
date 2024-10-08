interface StorageIconProperties {
    width?: number | string;
    height?: number | string;
    size?: number | string;
    fill?: string;
}

export default function Storage(props: StorageIconProperties) {
    const { width, height, size, fill } = props;
    const usedWidth = size || width || '11';
    const usedHeight = size || height || '12';
    const usedFill = fill || 'currentColor';

    return (
        <svg
            width={usedWidth}
            height={usedHeight}
            viewBox="0 0 11 12"
            fill="none"
            xmlns="http://www.w3.org/2000/svg"
        >
            <path
                fillRule="evenodd"
                clipRule="evenodd"
                d="M1.87951 1.48204C1.4802 1.88135 1.23467 2.46722 1.23467 3.2093V8.7907C1.23467 9.53278 1.4802 10.1187 1.87951 10.518C2.27881 10.9173 2.86468 11.1628 3.60676 11.1628H7.51374C8.25582 11.1628 8.8417 10.9173 9.241 10.518C9.6403 10.1187 9.88583 9.53278 9.88583 8.7907V3.2093C9.88583 2.46722 9.6403 1.88135 9.241 1.48204C8.8417 1.08274 8.25582 0.837209 7.51374 0.837209H3.60676C2.86468 0.837209 2.27881 1.08274 1.87951 1.48204ZM1.28751 0.890048C1.86495 0.312606 2.67443 0 3.60676 0H7.51374C8.44608 0 9.25555 0.312606 9.83299 0.890048C10.4104 1.46749 10.723 2.27697 10.723 3.2093V8.7907C10.723 9.72303 10.4104 10.5325 9.83299 11.11C9.25555 11.6874 8.44608 12 7.51374 12H3.60676C2.67443 12 1.86495 11.6874 1.28751 11.11C0.710067 10.5325 0.397461 9.72303 0.397461 8.7907V3.2093C0.397461 2.27697 0.710067 1.46749 1.28751 0.890048Z"
                fill={usedFill}
            />
            <path
                fillRule="evenodd"
                clipRule="evenodd"
                d="M0.397461 7.67442C0.397461 7.44323 0.584877 7.25581 0.816066 7.25581H10.3044C10.5356 7.25581 10.723 7.44323 10.723 7.67442C10.723 7.90561 10.5356 8.09302 10.3044 8.09302H0.816066C0.584877 8.09302 0.397461 7.90561 0.397461 7.67442Z"
                fill={usedFill}
            />
            <path
                fillRule="evenodd"
                clipRule="evenodd"
                d="M2.07188 6C2.07188 5.76881 2.2593 5.5814 2.49048 5.5814H3.04862C3.27981 5.5814 3.46723 5.76881 3.46723 6C3.46723 6.23119 3.27981 6.4186 3.04862 6.4186H2.49048C2.2593 6.4186 2.07188 6.23119 2.07188 6Z"
                fill={usedFill}
            />
            <path
                fillRule="evenodd"
                clipRule="evenodd"
                d="M2.07188 4.60465C2.07188 4.37346 2.2593 4.18605 2.49048 4.18605H3.04862C3.27981 4.18605 3.46723 4.37346 3.46723 4.60465C3.46723 4.83584 3.27981 5.02326 3.04862 5.02326H2.49048C2.2593 5.02326 2.07188 4.83584 2.07188 4.60465Z"
                fill={usedFill}
            />
            <path
                fillRule="evenodd"
                clipRule="evenodd"
                d="M2.07188 3.2093C2.07188 2.97811 2.2593 2.7907 2.49048 2.7907H3.04862C3.27981 2.7907 3.46723 2.97811 3.46723 3.2093C3.46723 3.44049 3.27981 3.62791 3.04862 3.62791H2.49048C2.2593 3.62791 2.07188 3.44049 2.07188 3.2093Z"
                fill={usedFill}
            />
            <path
                fillRule="evenodd"
                clipRule="evenodd"
                d="M7.37114 9.48837C7.37114 9.25718 7.55856 9.06977 7.78974 9.06977H7.79476C8.02595 9.06977 8.21336 9.25718 8.21336 9.48837C8.21336 9.71956 8.02595 9.90698 7.79476 9.90698H7.78974C7.55856 9.90698 7.37114 9.71956 7.37114 9.48837Z"
                fill={usedFill}
            />
        </svg>
    );
}