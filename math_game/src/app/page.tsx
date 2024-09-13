import Header from '@/components/Header';
import { SignedIn, UserButton } from '@clerk/nextjs';
import { currentUser } from '@clerk/nextjs/server';
import { redirect } from 'next/navigation';
import React from 'react'

const Home = async () => {
  const clerkUser = await currentUser();
  if (!clerkUser) redirect("/sign-in")

  return (
    <div>
      <Header className='sticky left-0 top-0'>
        <div className='flex items-center gap-2 lg:gap-4'>
          <SignedIn>
            <UserButton/>
          </SignedIn>
        </div>
      </Header>

    </div>
  )
}

export default Home 
