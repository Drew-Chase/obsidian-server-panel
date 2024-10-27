import {createContext, Dispatch, ReactNode, SetStateAction, useContext, useEffect, useState} from "react";
import Server from "../ts/servers.ts";

interface SelectedServerContextType
{
    selectedServerId: string | null;
    setSelectedServerId: Dispatch<SetStateAction<string | null>>;
    server: Server | null;
}

const SelectedServerContext = createContext<SelectedServerContextType | undefined>(undefined);

export function SelectedServerProvider({children}: { children: ReactNode })
{
    const [selectedServerId, setSelectedServerId] = useState<string | null>(localStorage.getItem("selectedServer"));
    const [server, setServer] = useState<Server | null>(null);
    useEffect(() =>
    {
        if (selectedServerId)
        {
            localStorage.setItem("selectedServer", selectedServerId);
            Server.get(selectedServerId).then(server =>
            {
                if (!server)
                {
                    localStorage.removeItem("selectedServer");
                    setSelectedServerId(null);
                    return;
                }
                setServer(server);
            }).catch(()=>{
                localStorage.removeItem("selectedServer");
                setSelectedServerId(null);
            });
        } else localStorage.removeItem("selectedServer");


    }, [selectedServerId]);

    return (
        <SelectedServerContext.Provider value={{selectedServerId, setSelectedServerId, server}}>
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