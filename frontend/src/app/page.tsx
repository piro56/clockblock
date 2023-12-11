"use client"
import DatePicker from "react-datepicker";
import { ChangeEvent, useState, MouseEvent, useEffect} from 'react';
import "react-datepicker/dist/react-datepicker.css";
import "./datepicker.css";
import { toast } from 'react-toastify';
import 'react-toastify/dist/ReactToastify.css';
import Locker from "@/components/locker";
import Unlocker from "@/components/unlocker";
export default function Home() {
  let [showLocker, setLocker] = useState(true);


  return (
    <main className="flex min-h-screen flex-col items-center justify-between justify-items-center pl-24 pr-24 pt-12">
      <div className="max-w-5xl w-full items-center content-center font-mono md:flex flex-col">
        <h1 className='align-center text-center justify-self-center text-3xl'>
          ClockBlock
        </h1>
        <p className='text-md pb-8'>An easy way to block access to your files and keys.</p>
        <div className='flex flex-row min-w-[40%] md:max-w-[45%] pt-4 
              text-center justify-items-center justify-content-center rounded-xl flex-wrap'>
          <div className='w-1/2 px-4'>
            <button type="button" onClick={(e) => setLocker(true)} 
              className='w-full text-center bg-red-800 rounded-md w-[50%] min-h-[40px]'>Lock</button>
          </div>
          <div className='w-1/2 px-4'>
            <button type="button"  onClick={(e) => setLocker(false)} 
            className='w-full text-center bg-green-500 rounded-md w-[50%] min-h-[40px]'>Unlock</button>
          </div>
        </div>
        {
          showLocker &&
          <Locker/>
        }
        {
          !showLocker &&
          <Unlocker/>
        }

      </div>
    </main>
  )
}
