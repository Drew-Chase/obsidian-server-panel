import {createContext, ReactNode, useContext, useEffect, useState} from "react";

interface ScreenSizeContextType
{
    width: number;
    height: number;
}

const ScreenSizeContext = createContext<ScreenSizeContextType | undefined>(undefined);

export function ScreenSizeProvider({children}: { children: ReactNode })
{
    const [width, setWidth] = useState(window.innerWidth);
    const [height, setHeight] = useState(window.innerHeight);

    useEffect(() =>
    {
        const handleResize = () =>
        {
            setWidth(window.innerWidth);
            setHeight(window.innerHeight);
        };
        window.addEventListener("resize", handleResize);
        return () => window.removeEventListener("resize", handleResize);
    }, []);

    return (
        <ScreenSizeContext.Provider value={{width, height}}>
            {children}
        </ScreenSizeContext.Provider>
    );
}

export function useScreenSize(): ScreenSizeContextType
{
    const context = useContext(ScreenSizeContext);
    if (!context)
    {
        throw new Error("useScreenSize must be used within a ScreenSizeProvider");
    }
    return context;
}