import $ from "jquery";
import {api_domain} from "../../main.tsx";

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
    private interval: number = 0;
    public current_data: SystemMonitorResponse = SystemMonitorResponseDefault;

    async ping(): Promise<SystemMonitorResponse>
    {
        return this.current_data = await $.get(`${api_domain}/system/usage`);
    }

    startMonitoring(update_interval: number, action: (data: SystemMonitorResponse) => void): void
    {
        this.interval = setInterval(async () =>
        {
            action(await this.ping());
        }, update_interval);
    }

    stopMonitoring(): void
    {
        clearInterval(this.interval);
    }

}