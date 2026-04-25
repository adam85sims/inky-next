export const snippets = [
  {
    category: "Structure",
    items: [
      { label: "Knot", content: "=== ${1:knot_name} ===\n${0}" },
      { label: "Stitch", content: "= ${1:stitch_name}\n${0}" },
      { label: "Include", content: "INCLUDE ${1:filename}.ink\n${0}" }
    ]
  },
  {
    category: "Content",
    items: [
      { label: "Choice", content: "* ${1:choice_text}\n    ${0}" },
      { label: "Sticky Choice", content: "+ ${1:choice_text}\n    ${0}" },
      { label: "Sequence", content: "{ ${1:item1} | ${2:item2} | ${3:item3} }${0}" },
      { label: "Cycle", content: "{& ${1:item1} | ${2:item2} | ${3:item3} }${0}" },
      { label: "Shuffle", content: "{~ ${1:item1} | ${2:item2} | ${3:item3} }${0}" }
    ]
  },
  {
    category: "Logic",
    items: [
      { label: "Global Variable", content: "VAR ${1:name} = ${2:0}\n${0}" },
      { label: "Local Variable", content: "~ temp ${1:name} = ${2:0}\n${0}" },
      { label: "Function", content: "=== function ${1:name} ===\n    ${0}" },
      { label: "Tunnel", content: "-> ${1:target} ->\n${0}" },
      { label: "Thread", content: "<- ${1:target}\n${0}" }
    ]
  }
];
