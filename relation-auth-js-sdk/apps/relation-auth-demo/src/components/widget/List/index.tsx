import React from 'react'
import ListHeader from './ListHeader'
import ListItem from './ListItem'
import './index.css'

const List = ({
  children,
  header,
}: {
  children?: React.ReactNode | React.ReactNode[]
  header?: string | React.ReactNode
}) => {
  return (
    <div className="list">
      {header && <ListHeader>{header}</ListHeader>}
      {children}
    </div>
  )
}

List.ListItem = ListItem

export default List
