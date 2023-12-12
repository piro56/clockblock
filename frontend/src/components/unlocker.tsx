import DatePicker from "react-datepicker";
import { ChangeEvent, useState, MouseEvent, useEffect } from 'react';
import "react-datepicker/dist/react-datepicker.css";
import "./datepicker.css";
import { toast } from 'react-toastify';
import 'react-toastify/dist/ReactToastify.css';

export default function Unlocker() {
 
    const [receipt, setReceipt] = useState("");
    const [unlockResult, setRetData] = useState("");
    const ip = process.env.NEXT_PUBLIC_BACKEND_IP;
    const port = process.env.NEXT_PUBLIC_BACKEND_PORT;
    const backendconn = 'http://' + ip + ':' + port + "/";
    const unlockpath = backendconn + 'decrypt/unlock';


    function onUnlock(e: MouseEvent) {
        console.log(unlockpath)
        try {
            fetch(unlockpath, {
                method: "POST",
                body: JSON.stringify({
                    "id": parseInt(receipt),
                }),
                headers: {
                    'Access-Control-Allow-Origin':'*',
                    "Content-type": "application/json; charset=UTF-8"
                }
            }).then(x => x.text())
                .then(data =>
                    setRetData(data.trim().replace(/[^\x00-\x7F]/g, ""))
                );
        } catch (e: any) {
            toast.error(e.toString());
        }
    }

    function lockCopy() {
        navigator.clipboard.writeText(unlockResult);
    }

    return (
        <div className='flex flex-row min-w-[40%] md:max-w-[45%] pt-4 
              text-center justify-items-center justify-content-center rounded-xl flex-wrap'>
            <div className='basis-full p-2 pb-1'>
                <input className='text-center bg-gray-800 rounded-md w-[100%] min-h-[40px]' placeholder='Receipt' value={receipt} onChange={(e) => setReceipt(e.target.value)}></input>
            </div>
            <form className='basis-full pl-4 pr-4 pt-1'>
                <button type="button" className='text-center bg-green-800 rounded-md w-[50%] min-h-[40px]' onClick={(e) => onUnlock(e)}>Unlock</button>
            </form>
            {

            unlockResult &&
                <>
                    <div className='basis-full pt-4 pl-4 pr-4'>
                        <button type="button" className='text-center bg-blue-800 rounded-md w-[50%] min-h-[40px]' onClick={(e) => lockCopy()}>Copy</button>
                    </div>
                    <div className="flex flex-col flex-wrap grow h-[300px] w-full p-2">
                        <div className="h-full w-full grow">
                            <div>
                                <p>Unencryption Result:</p>
                                <h1 className="rounded-md text-xl text-left overflow-hidden break-normal">{unlockResult}</h1>
                            </div>
                        </div>
                    </div>
                </>
            }
        </div>
    )

}