import React, { useState, useEffect } from "react"
import { Container } from "react-bulma-components"

import Parent from "./parent"
import Input from "./input"
import Table from "./table"
import Function from "./function"
import Return from "./return"

const Element = (props: {
  element: any
  handleElement: Function
  getElement: Function
  context: { parent: string; current: string }
  getSubflowName: Function
}) => {
  function elementSwitch() {
    switch (props.element.type) {
      case "parent":
        return (
          <Parent
            id={props.element.id}
            value={props.element.value}
            handleElement={props.handleElement}
            context={props.context}
            getSubflowName={props.getSubflowName}
          />
        )
      case "input":
        return (
          <Input
            id={props.element.id}
            value={props.element.value}
            handleElement={props.handleElement}
            context={props.context}
            getSubflowName={props.getSubflowName}
          />
        )
      case "table":
        return (
          <Table
            id={props.element.id}
            value={props.element.value}
            handleElement={props.handleElement}
          />
        )
      case "decision":
        return <p>Decision Table</p>
      case "function":
        return (
          <Function
            id={props.element.id}
            value={props.element.value}
            handleElement={props.handleElement}
            context={props.context}
            getSubflowName={props.getSubflowName}
          />
        )
      case "subflow":
        return <p>Subflow</p>
      case "return":
        return (
          <Return
            id={props.element.id}
            value={props.element.value}
            handleElement={props.handleElement}
            context={props.context}
            getSubflowName={props.getSubflowName}
          />
        )
      default:
        return <p>Error: Unknown Element Type</p>
    }
  }

  return (
    <Container>
      {props.element && elementSwitch()}
      <br />
    </Container>
  )
}

export default Element
