export default class Conversions
{
    static bytesToSize(bytes: number): string
    {
        const sizes = ["Bytes", "KB", "MB", "GB", "TB"];
        if (bytes === 0) return "0 Byte";
        const i = parseInt(String(Math.floor(Math.log(bytes) / Math.log(1024))));
        return Math.round(bytes / Math.pow(1024, i)) + " " + sizes[i];
    }

    static formatLargeNumber(value: number, decimal: number): string
    {
        const sizes = ["K", "M", "B", "T"];
        if (value === 0) return "0";
        if (value < 1000) return value.toString();
        const i = Math.floor(Math.log(value) / Math.log(1000));
        return (value / Math.pow(1000, i)).toFixed(decimal) + sizes[i - 1];
    }

    static formatTimeClosestRelative(date: Date): string
    {
        const seconds = Math.floor((new Date().getTime() - date.getTime()) / 1000);
        let interval = Math.floor(seconds / 31536000);
        if (interval > 1) return interval + " years ago";
        interval = Math.floor(seconds / 2592000);
        if (interval > 1) return interval + " months ago";
        interval = Math.floor(seconds / 86400);
        if (interval > 1) return interval + " days ago";
        interval = Math.floor(seconds / 3600);
        if (interval > 1) return interval + " hours ago";
        interval = Math.floor(seconds / 60);
        if (interval > 1) return interval + " minutes ago";
        return Math.floor(seconds) + " seconds ago";
    }
}