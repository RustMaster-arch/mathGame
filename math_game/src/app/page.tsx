import Header from '@/components/Header';
import { SignedIn, UserButton } from '@clerk/nextjs';
import { currentUser } from '@clerk/nextjs/server';
import { redirect } from 'next/navigation';
import React from 'react'

const Home = async () => {
  const clerkUser = await currentUser();
  if (!clerkUser) redirect("/sign-in")

  return (
    <div className='flex flex-col text-center'>
      <Header className='sticky left-0 top-0'>
        <div className='flex items-center gap-2 lg:gap-4'>
          <SignedIn>
            <UserButton/>
          </SignedIn>
        </div>
      </Header>

      <h1 className='h1'>Leader Board</h1>

    </div>
  )
}

export default Home 
