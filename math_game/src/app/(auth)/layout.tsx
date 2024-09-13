import React from 'react'

const layout = ({ children }: { children: React.ReactNode }) => {
  return (
    <div className='sign min-h-screen'>
      {children}
    </div>
  )
}

export default layout
