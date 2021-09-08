
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

import {
    BrowserRouter as Router,
    Switch,
    Route,
    useRouteMatch,
    useParams
  } from "react-router-dom";

const selectGlobal = (state) => state.global
const selectToolbar = (state) => state.toolbar

export function run(store, mount) {
    ReactDOM.render(
        <Provider store={store}><OverlayApp/></Provider>,
        mount
    );
  }

function OverlayApp() {
    return (
        <ThemeProvider colorMode="night">
            <Router>
            <MainApp/>
            </Router>
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
                <ButtonOutline as='a' href='/'>Experiments</ButtonOutline>
                <ButtonOutline as='a' href='https://cosmoplankton.studio'>Home</ButtonOutline>
                <ButtonOutline as='a' href='https://blog.cosmoplankton.studio'>Blog</ButtonOutline>
            </ButtonGroup>
        </Box>
    )
}


function MainApp() {

    // const { path } = useRouteMatch();
    // console.log('log from MainApp<>' + path)
    // const { extn } = useParams();

    return (
        <Switch>
          <Route path={["/webgl", "/webgl.html"]}>
            <AppToolBar path={'webgl'}/>
          </Route>
          <Route path={["/webgpu", "/webgpu.html"]}>
            <AppToolBar path={'webgpu'}/>
          </Route>
          <Route exact path="/">
            <AppToolBar path={'index'}/>
          </Route>
        </Switch>
    )
}

function WeblButtonGroup(ctx) {
    return(
        <Box display="flex">
            <ButtonGroup display='block' m={2}>
                <ClickButton text='Pause' onClick={ctx.handlePause}></ClickButton>
                <ClickButton text='Start' onClick={ctx.handleUnPause}></ClickButton>
            </ButtonGroup>
        </Box>
    )
}

function DefaultButtonGroup(ctx) {
    return (
        <Box display="flex"/>
    )
}

function WebgpuButtonGroup(ctx) {
    return (
        <Box display="flex">
            <ButtonGroup display='block' m={2}>
                <ClickButton text='SliceGlobal/Alert' onClick={ctx.handleClickGlobal}></ClickButton>
                <ClickButton text='SliceTools/Alert' onClick={ctx.handleClickToolbar}></ClickButton>
                <ClickButton text='Hello' onClick={ctx.handleMsgHello}></ClickButton>
                <ClickButton text='Hi' onClick={ctx.handleMsgHi}></ClickButton>
            </ButtonGroup>
            <TextLabel text={ctx.globalStates.message}></TextLabel>
            <TextLabel text={ctx.toolbarStates.message}></TextLabel>
        </Box>
    )
}

function AppToolBar(props) {

    const dispatch = useDispatch()
    const ctx = {
        globalStates: useSelector(selectGlobal),
        toolbarStates: useSelector(selectToolbar),
        handleClickGlobal: ()=>{dispatch({ type: 'global/log', payload: '' })},
        handleClickToolbar: ()=>{dispatch({ type: 'toolbar/log', payload: '' })},
        handlePause: ()=>{dispatch({ type: 'global/pause', payload: '' })},
        handleUnPause: ()=>{dispatch({ type: 'global/unpause', payload: '' })},
        handleMsgHello: ()=>{
            dispatch({ type: 'global/message/hello', payload: '' });
            dispatch({ type: 'toolbar/message/hello', payload: '' });
        },
        handleMsgHi: ()=>{
            dispatch({ type: 'global/message/hi', payload: '' });
            dispatch({ type: 'toolbar/message/hi', payload: '' });
        }
    }

    return(
        <Box
        display="flex"
        textShadow="0px 0px" 
        p={0}
        m={0}
        bg="bg.primary">

            {props.path == 'webgl' && <WeblButtonGroup {...ctx}/>}
            {props.path == 'index' && <DefaultButtonGroup {...ctx}/>}
            {props.path == 'webgpu' && <WebgpuButtonGroup {...ctx}/>}

            <Box flexGrow={1}></Box>
            <StudioLinks/>
    
        </Box>
    )
}
