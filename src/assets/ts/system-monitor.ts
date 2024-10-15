export interface SystemMonitorResponse
{
    cores: number[];
    cpu_usage: number;
    memory: {
        free: number;
        swap_free: number;
        swap_total: number;
        swap_used: number;
        total: number;
        used: number;
    };
}

export const SystemMonitorResponseDefault: SystemMonitorResponse = {
    cores: [],
    cpu_usage: 0,
    memory: {
        free: 0,
        swap_free: 0,
        swap_total: 0,
        swap_used: 0,
        total: 0,
        used: 0
    }
};


export default class SystemMonitor
{
    public current_data: SystemMonitorResponse = SystemMonitorResponseDefault;
    private socket: WebSocket | null = null;

    startMonitoring(action: (data: SystemMonitorResponse) => void): void
    {
        this.socket = new WebSocket("/api/system/usage/ws");
        this.socket.addEventListener("open", () =>
        {
            console.log("Connected to system monitor websocket");
        });
        this.socket.addEventListener("message", (event) =>
        {
            const data = JSON.parse(event.data);
            action(data);
        });
        this.socket.addEventListener("close", () =>
        {
            console.log("Disconnected from system monitor websocket");
        });
        this.socket.addEventListener("error", (event) =>
        {
            console.error("Error on system monitor websocket", event);
        });
    }

    stopMonitoring(): void
    {
        this.socket?.close();
    }


}