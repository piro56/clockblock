"use client"
import DatePicker from "react-datepicker";
import { useState } from 'react';
import "react-datepicker/dist/react-datepicker.css";
import "./datepicker.css";


export default function Home() {
  const [startDate, setStartDate] = useState(new Date());


  return (
    <main className="flex min-h-screen flex-col items-center justify-between justify-items-center pl-24 pr-24 pt-12">
      <div className="max-w-5xl w-full items-center content-center font-mono md:flex flex-col">
        <h1 className='align-center text-center justify-self-center text-3xl'>
          ClockBlock
        </h1>
        <p className='text-md pb-8'>An easy way to block access to your files and keys.</p>
        <div className='flex flex-row min-w-[40%] md:max-w-[45%] pt-4 
          text-center justify-items-center justify-content-center rounded-xl flex-wrap'>
           <form className='basis-full p-2 pb-1'>
          <input className='text-center bg-gray-800 rounded-md w-[100%] min-h-[40px]' placeholder='Key'></input>
          </form>
          <form className='basis-full p-2 pb-1'>
          <input className='text-center bg-gray-800 rounded-md w-[100%] min-h-[40px]' placeholder='Nonce (Optional)'></input>
          </form>
          <div className="flex flex-row grow align-center justify-items-center p-2 gap-2">
            <div className="flex flex-col hidden md:flex justify-center grow text-center bg-gray-800 rounded"
             title="You will not be able to retrieve your key (and nonce) until after this timestamp.">
              <h1>Unlock Time:</h1>
            </div>
            <form className='grow min-h-[40px] '>
              <DatePicker dateFormat="MM/dd/yyyy HH:mm" selected={startDate} onChange={ (date) => {if (date) setStartDate(date)}} showTimeSelect/>
            </form>
          </div>

          <form className='basis-full'>
          <button className='text-center bg-green-800 rounded-md w-[50%] min-h-[40px]'>Submit</button>
          </form>
        </div>
      </div>
    </main>
  )
}
