import { useState } from 'react'

import LoginButton from './components/ui/LoginButton'
import SvgIcon from './components/widget/SvgIcon'

import './App.css'

function App() {
    const [address, setAddress] = useState<string | undefined>()

    return (
        <div className="Web3-App">
            <div className="ui-base header">
                <SvgIcon className="header-logo" name="eth" />
                <div className="header-title">Relation Web3 App</div>
                <LoginButton
                    onSuccess={(_address: string) => {
                        setAddress(_address)
                    }}
                />
            </div>
            <div className="content">

            </div>
        </div>
    )
}

export default App
