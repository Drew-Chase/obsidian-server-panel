import {createContext, ReactNode, useContext, useEffect, useState} from "react";
import $ from 'jquery';


export type Notification = {
    // Unique identifier for the notification
    id: string;

    // Title of the notification
    title: string;

    // Message body of the notification
    message: string;

    // Indicates if the notification has been read
    read: boolean;

    // Indicates if the notification has been archived
    archived: boolean;

    // Action associated with the notification
    action: NotificationAction[];

    // The id of the sender, it could be a user id or a server id
    sender: string;

    // The type of the sender, it could be a user, server or system
    senderType: "user" | "server" | "system";

    // The date the notification was sent
    date: Date;
}

export type NotificationAction = {
    label: string;
    command: string;
    color: string;
}

interface NotificationContextType
{
    notifications: Notification[];
    markAsRead: (id: string) => void;
    markAsUnread: (id: string) => void;
    archive: (id: string) => void;
}

const NotificationContext = createContext<NotificationContextType | undefined>(undefined);

export function NotificationProvider({children}: { children: ReactNode })
{
    const [notifications, setNotification] = useState([] as Notification[]);


    useEffect(() =>
    {
        const notificationsServerSideEvent = new EventSource("/api/notifications");
        notificationsServerSideEvent.onopen = () => console.log("Connected to notifications server side event");
        notificationsServerSideEvent.onerror = () => console.error("Error connecting to notifications server side event");
        notificationsServerSideEvent.addEventListener("full", e =>
        {
            const notifications = JSON.parse(e.data) as Notification[];
            setNotification(notifications);
        });

        return () =>
        {
            notificationsServerSideEvent.close();
        };
    }, []);

    return (
        <NotificationContext.Provider value={{
            notifications,
            archive: (id: string) =>{
                $.post(`/api/notifications/archive/${id}`);
            },
            markAsRead: (id: string) =>{
                $.post(`/api/notifications/read/${id}`);
            },
            markAsUnread: (id: string) =>{
                $.post(`/api/notifications/unread/${id}`);
            }

        }}>
            {children}
        </NotificationContext.Provider>
    );
}

export function useNotification(): NotificationContextType
{
    const context = useContext(NotificationContext);
    if (!context)
    {
        throw new Error("useNotification must be used within a NotificationProvider");
    }
    return context;
}