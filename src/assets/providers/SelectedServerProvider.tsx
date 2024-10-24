import {createContext, Dispatch, ReactNode, SetStateAction, useContext, useEffect, useState} from "react";

interface SelectedServerContextType
{
    selectedServerId: string | null;
    setSelectedServerId: Dispatch<SetStateAction<string | null>>;
}

const SelectedServerContext = createContext<SelectedServerContextType | undefined>(undefined);

export function SelectedServerProvider({children}: { children: ReactNode })
{
    const [selectedServerId, setSelectedServerId] = useState<string | null>(localStorage.getItem("selectedServer"));
    useEffect(() =>
    {
        if (selectedServerId) localStorage.setItem("selectedServer", selectedServerId);
        else localStorage.removeItem("selectedServer");

    }, [selectedServerId]);
    return (
        <SelectedServerContext.Provider value={{selectedServerId, setSelectedServerId}}>
            {children}
        </SelectedServerContext.Provider>
    );
}

export function useSelectedServer(): SelectedServerContextType
{
    const context = useContext(SelectedServerContext);
    if (!context)
    {
        throw new Error("useSelectedServer must be used within a SelectedServerProvider");
    }
    return context;
}