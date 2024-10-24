import {createContext, ReactNode, useContext, useState} from "react";
import {Button, ButtonProps, Modal, ModalBody, ModalContent, ModalFooter, ModalHeader} from "@nextui-org/react";

export interface AlertOptions
{
    title: string;
    message: string;
    type: "error" | "warning" | "info" | "success";
    actions?: AlertActions[];
}

export interface AlertActions extends ButtonProps
{
    label: string;
}

interface AlertModalContextType
{
    alert: (arg0: AlertOptions) => void;
}

const AlertModalContext = createContext<AlertModalContextType | undefined>(undefined);

export function AlertModalProvider({children}: { children: ReactNode })
{
    const [isOpen, setIsOpen] = useState<boolean>(false);
    const [options, setOptions] = useState<AlertOptions>({
        title: "",
        message: "",
        type: "info"
    });
    const alert = (options: AlertOptions) =>
    {
        setOptions(options);
        setIsOpen(true);
    };
    return (
        <AlertModalContext.Provider value={{alert}}>
            <Modal isOpen={isOpen} onClose={() =>
            {
                setOptions({} as AlertOptions);
                setIsOpen(false);
            }}>
                <ModalContent>
                    {onClose => (
                        <>
                            <ModalHeader>{options.title}</ModalHeader>
                            <ModalBody>
                                <p>{options.message}</p>
                            </ModalBody>
                            <ModalFooter>
                                {options.actions?.map((action, index) => (
                                    <Button key={index} {...action} onClick={e =>
                                    {
                                        action.onClick?.(e);
                                        onClose();
                                    }}>
                                        {action.label}
                                    </Button>
                                ))}
                            </ModalFooter>
                        </>
                    )}
                </ModalContent>
            </Modal>
            {children}
        </AlertModalContext.Provider>
    );
}

export function useAlertModal(): AlertModalContextType
{
    const context = useContext(AlertModalContext);
    if (!context)
    {
        throw new Error("useAlertModal must be used within a AlertModalProvider");
    }
    return context;
}