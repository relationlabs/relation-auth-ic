import React from 'react'
import './ListHeader.css'

const ListHeader = ({
  children,
}: {
  children?: React.ReactNode | React.ReactNode[]
}) => {
  return <div className="list-header">{children}</div>
}

export default ListHeader
