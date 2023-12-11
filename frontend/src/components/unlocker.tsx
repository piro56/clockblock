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
                    "Content-type": "application/json; charset=UTF-8"
                }
            }).then(x => x.text())
                .then(data =>
                    setRetData(data)
                );
        } catch (e: any) {
            toast.error(e.toString());
        }

        console.log(unlockResult);

    }

    return (
        <div className='flex flex-row min-w-[40%] md:max-w-[45%] pt-4 
              text-center justify-items-center justify-content-center rounded-xl flex-wrap'>
            <form className='basis-full p-2 pb-1'>
                <input className='text-center bg-gray-800 rounded-md w-[100%] min-h-[40px]' placeholder='Receipt' value={receipt} onChange={(e) => setReceipt(e.target.value)}></input>
            </form>
            <div className='basis-full pl-4 pr-4 pt-1'>
                <button type="button" className='text-center bg-green-800 rounded-md w-[50%] min-h-[40px]' onClick={(e) => onUnlock(e)}>Unlock</button>
            </div>
        </div>
    )

}