interface Poll {
    id: string,
    title: string,
    createTime: string,
    creator: string,
    options: {id: string, text: string}[]
}

export default Poll
