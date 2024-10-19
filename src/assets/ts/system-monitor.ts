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
    private socket: EventSource | null = null;

    startMonitoring(action: (data: SystemMonitorResponse) => void): void
    {
        this.socket = new EventSource("/api/system/usage/sse");
        this.socket.addEventListener("open", () =>
        {
            console.log("System Monitor SSE Connected");
        });
        this.socket.addEventListener("system_usage", (event) =>
        {
            const data = JSON.parse(event.data);
            this.current_data = data;
            action(data);
        });
        this.socket.addEventListener("error", (event) =>
        {
            console.error("System Monitor SSE Error", event);
        });
        this.socket.addEventListener("close", () =>
        {
            console.log("System Monitor SSE Closed");
        });
    }

    stopMonitoring(): void
    {
        console.log("System Monitor SSE Stopped");
        this.socket?.close();
    }


}