import React from 'react'
import './ListItem.css'

const ListItem = ({
  children,
  actions,
}: {
  children?: React.ReactNode | React.ReactNode[]
  actions?: React.ReactNode[]
}) => {
  return (
    <div className="list-item ellipsis">
      <div className="list-item-content">
        <span>{children}</span>
      </div>
      {Array.isArray(actions) && actions.length && (
        <div className="list-item-actions">
          {actions.map((action, index) => {
            return <React.Fragment key={index}>{action}</React.Fragment>
          })}
        </div>
      )}
    </div>
  )
}

export default ListItem
