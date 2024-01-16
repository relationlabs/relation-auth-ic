import React from 'react'
import './index.css'

const Button = ({
  children,
  className = '',
  onClick = () => {},
}: {
  children?: React.ReactNode | React.ReactNode[]
  className?: String
  onClick?: Function
}) => {
  return (
    <div
      className={`ui-base btn ${className || ''}`}
      onClick={() => {
        if (typeof onClick === 'function') onClick()
      }}
    >
      {children}
    </div>
  )
}

export default Button
