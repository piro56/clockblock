"use client"
import DatePicker from "react-datepicker";
import { useState } from 'react';
import "react-datepicker/dist/react-datepicker.css";
import "./datepicker.css";

export default function Home() {
  const [startDate, setStartDate] = useState(new Date());


  return (
    <main className="flex min-h-screen flex-col items-center justify-between justify-items-center pl-24 pr-24 pt-12">
      <div className="max-w-5xl w-full items-center content-center font-mono lg:flex flex-col">
        <h1 className='align-center text-center justify-self-center text-3xl'>
          ClockBlock
        </h1>
        <p className='text-md pb-8'>An easy way to block access to your files and keys.</p>
        <div className='flex flex-row min-w-[40%] lg:max-w-[45%] pt-8 
          text-center justify-items-center justify-content-center rounded-xl flex-wrap'>
 
           <form className='basis-full p-2'>
          <input className='text-center bg-gray-800 rounded-md w-[100%] min-h-[40px]' placeholder='Key'></input>
          </form>
          <form className='grow p-2 min-h-[40px]'>
            <DatePicker dateFormat="MM/dd/yyyy HH:mm" selected={startDate} onChange={ (date) => {if (date) setStartDate(date)}} showTimeSelect/>
          </form>
          <form className='basis-full p-2 pt-4'>
          <button className='text-center bg-green-800 rounded-md w-[50%] min-h-[40px]'>Submit</button>
          </form>
          <p>{}</p>
        </div>
      </div>
    </main>
  )
}
