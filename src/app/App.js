
import React from 'react';
import ReactDOM from 'react-dom';
import { Provider, useDispatch, useSelector } from 'react-redux'
import {
    ThemeProvider,
    Box,
    Button,
    ButtonOutline,
    BranchName,
    ButtonGroup
} from '@primer/components'

import store from './store'


const selectGlobal = (state) => state.global
const selectToolbar = (state) => state.toolbar

export function run() {
    ReactDOM.render(
        <Provider store={store}><MyApp/></Provider>,
        document.getElementById('react-app')
    );
  }

function MyApp() {
    return (
        <ThemeProvider colorMode="night">
            <MainAppBar/>
        </ThemeProvider>
    )
}

function ClickButton(props) {
    return (
    <Button m={0} onClick={props.onClick}>{props.text}</Button>
    )
}

function TextLabel(props) {
    return (
        <BranchName variant="small" sx={{my: 3, mx:1, px: 2}}>{props.text}</BranchName>
    )
}

function StudioLinks() {
    return (
        <Box mr={2}>
            <ButtonGroup display='block' my={2}>
                <ButtonOutline as='a' href='https://cosmoplankton.studio'>Home</ButtonOutline>
                <ButtonOutline as='a' href='https://blog.cosmoplankton.studio'>Blog</ButtonOutline>
                <ButtonOutline as='a' href='/webgl.html'>Webgl</ButtonOutline>
            </ButtonGroup>
        </Box>
    )
}

function MainAppBar() {
    const dispatch = useDispatch()
    const globalStates = useSelector(selectGlobal)
    const toolbarStates = useSelector(selectToolbar)
    const handleClickGlobal = ()=>{dispatch({ type: 'global/log', payload: '' })}
    const handleClickToolbar = ()=>{dispatch({ type: 'toolbar/log', payload: '' })}
    const handlePause = ()=>{dispatch({ type: 'global/pause', payload: '' })}
    const handleUnPause = ()=>{dispatch({ type: 'global/unpause', payload: '' })}
    const handleMsgHello = ()=>{
        dispatch({ type: 'global/message/hello', payload: '' });
        dispatch({ type: 'toolbar/message/hello', payload: '' });
    }
    const handleMsgHi = ()=>{
        dispatch({ type: 'global/message/hi', payload: '' });
        dispatch({ type: 'toolbar/message/hi', payload: '' });
    }

return(
    <Box
    display="flex"
    textShadow="0px 0px" 
    p={0}
    m={0}
    bg="bg.primary">
        <Box
        display="flex"
        >
            <ButtonGroup display='block' m={2}>
                <ClickButton text='Global Action Alert' onClick={handleClickGlobal}></ClickButton>
                <ClickButton text='Toolbar Action Alert' onClick={handleClickToolbar}></ClickButton>
                <ClickButton text='Pause' onClick={handlePause}></ClickButton>
                <ClickButton text='Start' onClick={handleUnPause}></ClickButton>
                <ClickButton text='Hello' onClick={handleMsgHello}></ClickButton>
                <ClickButton text='Hi' onClick={handleMsgHi}></ClickButton>
            </ButtonGroup>
            <TextLabel text={globalStates.message}></TextLabel>
            <TextLabel text={toolbarStates.message}></TextLabel>
        </Box>
        <Box flexGrow={1}></Box>
        <StudioLinks/>

    </Box>)
}
