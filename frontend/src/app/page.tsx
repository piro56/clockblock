"use client"
import DatePicker from "react-datepicker";
import { ChangeEvent, useState } from 'react';
import "react-datepicker/dist/react-datepicker.css";
import "./datepicker.css";


export default function Home() {
  const [startDate, setStartDate] = useState(new Date());
  const [selectorToggle, setSelector] = useState(false);
  const [selectTimestamp, setTimestamp] = useState(0);
  const [timeInput, setTimeInput] = useState("");

  function onDatePick(date: Date | null) {
    if (!date) return;
    setStartDate(date);
    setTimestamp(date.getUTCDate());
    setTimeInput((date.getTime()/1000).toString());
  }
  function onTimestampPick(s: string) {
    let time = parseInt(s);
    time = time * 1000;
    if (isNaN(time)) {
      console.log("IsNan.");
      return;
    }
    let dateTime = new Date(time);
    setTimestamp(time);
    setStartDate(dateTime);
    setTimeInput(s);
    console.log(dateTime);
  }

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
              <button onClick={(e) => {setSelector(!selectorToggle);}}>{selectorToggle ? "Unlock Date:" : "Unlock Timestamp:"}</button>
            </div>
            { selectorToggle && 
              <form className='grow min-h-[40px] '>
                <DatePicker dateFormat="MM/dd/yyyy HH:mm" selected={startDate} onChange={(date) => onDatePick(date)} showTimeSelect/>
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

          <form className='basis-full'>
          <button className='text-center bg-green-800 rounded-md w-[50%] min-h-[40px]'>Submit</button>
          </form>
        </div>
      </div>
    </main>
  )
}
