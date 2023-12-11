import DatePicker from "react-datepicker";
import { ChangeEvent, useState, MouseEvent, useEffect } from 'react';
import "react-datepicker/dist/react-datepicker.css";
import "./datepicker.css";
import { toast } from 'react-toastify';
import 'react-toastify/dist/ReactToastify.css';

export default function Locker() {
    const [startDate, setStartDate] = useState(new Date());
    const [selectorToggle, setSelector] = useState(true);
    const [keyInput, setKeyInput] = useState("");
    const [selectTimestamp, setTimestamp] = useState(0);
    const [timeInput, setTimeInput] = useState("");
    const [lockOutput, setLockOutput] = useState("");
    const ip = process.env.NEXT_PUBLIC_BACKEND_IP;
    const port = process.env.NEXT_PUBLIC_BACKEND_PORT;
    const backendconn = 'http://' + ip + ':' + port + "/";
    const lockpath = backendconn + 'encrypt/lock';


    function onDatePick(date: Date | null) {
        if (!date) return;
        setStartDate(date);
        setTimestamp(date.getUTCDate());
        setTimeInput((date.getTime() / 1000).toString());
    }
    function onTimestampPick(s: string) {
        let time = parseInt(s);
        time = time * 1000;
        if (isNaN(time)) {
            return;
        }
        let dateTime = new Date(time);
        setTimestamp(time);
        setStartDate(dateTime);
        setTimeInput(s);
    }

    function onSubmit(e: MouseEvent) {
        let now = new Date();
        if (startDate < now) {
            toast("Bad Time!");
            return;
        }
        try {
            fetch(lockpath, {
                method: "PUT",
                body: JSON.stringify({
                    "data": keyInput,
                    "lock_time": (startDate.getTime() / 1000).toString()
                }),
                headers: {
                    "Content-type": "application/json; charset=UTF-8"
                }
            }).then(x => x.text())
                .then(data =>
                    setLockOutput(data)
                );

        } catch (e: any) {
            toast.error(e.toString());
        }
    }

    function lockCopy() {
        navigator.clipboard.writeText(lockOutput);
    }

    return (
        <div className='flex flex-row min-w-[40%] md:max-w-[45%] pt-4 
              text-center justify-items-center justify-content-center rounded-xl flex-wrap'>
            <form className='basis-full p-2 pb-1'>
                <input className='text-center bg-gray-800 rounded-md w-[100%] min-h-[40px]' placeholder='Key' value={keyInput} onChange={(e) => setKeyInput(e.target.value)}></input>
            </form>
            <form className='basis-full p-2 pb-1'>
                <input className='text-center bg-gray-800 rounded-md w-[100%] min-h-[40px]' placeholder='Nonce (Optional)'></input>
            </form>
            <div className="flex flex-row grow align-center justify-items-center p-2 gap-2">
                <div className="flex flex-col hidden md:flex justify-center grow text-center bg-gray-800 rounded"
                    title="You will not be able to retrieve your key (and nonce) until after this timestamp.">
                    <button type="button" onClick={(e) => { setSelector(!selectorToggle); }}>{selectorToggle ? "Unlock Date:" : "Unlock Timestamp:"}</button>
                </div>
                {selectorToggle &&
                    <form className='grow min-h-[40px] '>
                        <DatePicker dateFormat="MM/dd/yyyy HH:mm" selected={startDate} onChange={(date) => onDatePick(date)} showTimeSelect />
                    </form>
                }
                {
                    !selectorToggle &&
                    <form className='grow min-h-[40px] '>
                        <input className='text-center bg-gray-800 rounded-md w-[100%] min-h-[40px]' placeholder='Timestamp'
                            onInput={(e: any) => onTimestampPick(e.target.value)} value={timeInput}></input>
                    </form>
                }
            </div>
            <form className='basis-full pl-4 pr-4'>
                <button type="button" className='text-center bg-green-800 rounded-md w-[50%] min-h-[40px]' onClick={(e) => onSubmit(e)}>Submit</button>
            </form>


            {
                lockOutput &&
                <>
                    <div className='basis-full pt-4 pl-4 pr-4'>
                        <button type="button" className='text-center bg-blue-800 rounded-md w-[50%] min-h-[40px]' onClick={(e) => lockCopy()}>Copy</button>
                    </div>
                    <div className="flex flex-col grow h-[300px] w-full p-2">
                        <div className="h-full w-full grow">
                            <div>
                                <p>Your Receipt:</p>
                                <h1 className="rounded-md text-4xl">{lockOutput}</h1>
                            </div>
                        </div>
                    </div>
                </>
            }
        </div>
    )

}