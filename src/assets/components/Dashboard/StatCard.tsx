import {ReactElement, useEffect, useState} from "react";
import {Button, Progress} from "@nextui-org/react";
import {FontAwesomeIcon} from "@fortawesome/react-fontawesome";
import {faChartLine} from "@fortawesome/free-solid-svg-icons";
import {CartesianGrid, Line, LineChart, ResponsiveContainer, Tooltip as ChartTooltip, YAxis} from "recharts";

interface StatCardProps
{
    title?: string;
    value?: number;
    valueDisplay?: string;
    maxValue?: number;
    maxValueDisplay?: string;
    icon?: ReactElement;
    valueFormater?: (value: number) => string;
}


export default function StatCard(props: StatCardProps)
{
    const [values, setValues] = useState<number[]>(Array.from({length: 50}, () => 0));
    const [showGraph, setShowGraph] = useState(localStorage.getItem(`${props.title}-showGraph`) === "true");

    useEffect(() =>
    {
        if (props.value)
        {
            let tmp = [...values];
            if (tmp.length >= 50)
            {
                tmp.shift();
            }
            setValues([...tmp, props.value]);
        }
    }, [props.value]);

    useEffect(() =>
    {
        localStorage.setItem(`${props.title}-showGraph`, showGraph.toString());
    }, [showGraph]);

    return (
        <div className={"flex flex-col bg-neutral-600 rounded-2xl shadow-lg p-8 max-w-md min-w-sm grow shrink relative"}>
            <Button
                className={"absolute right-1 top-1 min-w-0"}
                variant={"light"}
                onClick={() => setShowGraph(prev => !prev)}
            >
                <FontAwesomeIcon icon={faChartLine}/>
            </Button>
            {showGraph ? (
                <>
                    <div className={"flex flex-row w-full items-center mb-3"}>
                        <span className={"text-primary"}>{props.icon}</span>
                        <p className={"text-tiny font-medium ml-2 mr-auto"}>{props.title}</p>
                    </div>
                    <ResponsiveContainer width={"100%"} height={"100%"} className={"-translate-x-8"}>
                        <LineChart
                            width={400}
                            height={160}
                            margin={{
                                top: 10,
                                right: 0,
                                left: 0,
                                bottom: 0
                            }}
                            data={values.map((value, index) => ({value, index}))}
                        >
                            {/* @ts-ignore */}
                            <ChartTooltip content={(props: CustomTooltipProps) => <CustomTooltip {...props}/>} animationDuration={200}/>
                            {props.maxValue ? <YAxis domain={[0, props.maxValue]}/> : <YAxis/>}
                            <CartesianGrid strokeOpacity={.2} vertical={false}/>
                            <Line type="monotone" dataKey="value" stroke="#8884d8" strokeWidth={2} isAnimationActive={false} dot={false}/>
                        </LineChart>
                    </ResponsiveContainer>

                </>
            ) : (
                <>
                    <div className={"flex flex-row w-full items-center mb-3"}>
                        <span className={"text-primary"}>{props.icon}</span>
                        <p className={"text-tiny font-medium ml-2 mr-auto"}>{props.title}</p>
                    </div>
                    <p className={"text-4xl font-semibold mb-4"}>{props.valueDisplay}</p>
                    {props.maxValue && (
                        <>
                            <p className={"text-sm text-neutral-500 ml-auto"}>of {props.maxValueDisplay}</p>
                            <Progress
                                aria-label={props.title}
                                size={"sm"}
                                value={props.value}
                                maxValue={props.maxValue}
                            />
                        </>
                    )}
                </>
            )}
        </div>
    );
}

interface CustomTooltipProps
{
    active: boolean;
    payload: Payload[];
    label: string;
}

interface Payload
{
    name: string;
    value: number;
}

const CustomTooltip = ({active, payload}: CustomTooltipProps) =>
{
    if (active && payload && payload.length)
    {
        return (
            <div className="bg-neutral-800 shadow-xl p-4 rounded-md flex flex-col w-[50px] shrink-0 grow justify-center items-center">
                {

                    Array.from(payload).map((i, n) =>
                    {
                        return (
                            <div key={`${i.name}-${i.value}-${n}`}>
                                <p className={"flex flex-row"}>
                                    <span className="font-bold">{i.value}</span>
                                </p>
                            </div>
                        );
                    })
                }
            </div>
        );
    }

    return null;
};