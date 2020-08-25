import React from "react"

import {
  faCircle,
  faArrowRight,
  faLock,
  faQuestion,
} from "@fortawesome/free-solid-svg-icons"

import ElementCard from "./ElementCard"
import Parent from "./Parent"
import Input from "./Input"
import Row from "./Row"
import Decision from "./Decision"
import Function from "./Function"
import Subflow from "./Subflow"
import Chart from "./Chart"
import Output from "./Output"

const Element = (props: {
  element: any
  handleElement: Function
  handleVariable: Function
  getVariable: Function
  context: { parent: string; current: string }
  getSubflow: Function
}) => {
  function elementSwitch() {
    switch (props.element.type) {
      case "parent":
        return (
          <ElementCard
            icon={faCircle}
            name={props?.getSubflow(props.context?.parent)?.name}
          >
            <Parent />
          </ElementCard>
        )
      case "input":
        return (
          <ElementCard
            icon={faArrowRight}
            name={`Input to ${
              props?.getSubflow(props.context?.current)?.name
            } Subflow`}
          >
            <Input
              id={props.element.id}
              value={props.element.value}
              handleElement={props.handleElement}
              handleVariable={props.handleVariable}
              getVariable={props.getVariable}
              context={props.context}
              getSubflow={props.getSubflow}
            />
          </ElementCard>
        )
      case "row":
        const row = props.getVariable(props.element.value?.id)
        return (
          <ElementCard icon={faLock} name={row?.value?.name}>
            <Row
              id={props.element.id}
              row={row}
              handleElement={props.handleElement}
              handleVariable={props.handleVariable}
              getVariable={props.getVariable}
            />
          </ElementCard>
        )
      case "decision":
        const value = props.element.value
        return (
          <ElementCard icon={faQuestion} name={value?.name}>
            <Decision
              id={props.element.id}
              value={value}
              handleElement={props.handleElement}
              handleVariable={props.handleVariable}
              getVariable={props.getVariable}
            />
          </ElementCard>
        )
      case "function":
        return (
          <Function
            id={props.element.id}
            value={props.element.value}
            handleElement={props.handleElement}
            handleVariable={props.handleVariable}
            getVariable={props.getVariable}
          />
        )
      case "subflow":
        return (
          <Subflow
            id={props.element.id}
            value={props.element.value}
            handleElement={props.handleElement}
            handleVariable={props.handleVariable}
            getVariable={props.getVariable}
            getSubflow={props.getSubflow}
          />
        )
      case "chart":
        return (
          <Chart
            id={props.element.id}
            value={props.element.value}
            handleElement={props.handleElement}
            handleVariable={props.handleVariable}
            getVariable={props.getVariable}
          />
        )
      case "output":
        return (
          <Output
            id={props.element.id}
            value={props.element.value}
            handleElement={props.handleElement}
            handleVariable={props.handleVariable}
            getVariable={props.getVariable}
            context={props.context}
            getSubflow={props.getSubflow}
          />
        )
      default:
        return <h4>Error: Unknown Element Type</h4>
    }
  }

  return <React.Fragment>{props.element && elementSwitch()}</React.Fragment>
}

export default Element
