
                // Generated by golem-scalajs-wit-bindgen
                package tests

                import scala.scalajs.js
                import scala.scalajs.js.JSConverters._

                
                sealed trait Message extends js.Object {
                    val messageId: Integer
val userId: Integer
val content: String
val channel: MessageChannel
val status: MessageStatus
                }
                object Message {
                    def apply(messageId: Integer, userId: Integer, content: String, channel: MessageChannel, status: MessageStatus): Message = {
                        val messageId0: Integer = messageId
val userId0: Integer = userId
val content0: String = content
val channel0: MessageChannel = channel
val status0: MessageStatus = status

                        new Message {
                            val messageId: Integer = messageId0
val userId: Integer = userId0
val content: String = content0
val channel: MessageChannel = channel0
val status: MessageStatus = status0
                        }
                    }
                }
            

                
                sealed trait MessageResult extends js.Object { self =>
                    type Type

                    val tag: String
                    val `val`: js.UndefOr[Type]
                }

                object MessageResult {
                    
                            def success(value: Message) = new MessageResult {
                                type Type = Message
                                
                                val tag: String = "success"
                                override val `val`: js.UndefOr[Type] = value
                            }
                        

                            def failure(value: MessageError) = new MessageResult {
                                type Type = MessageError
                                
                                val tag: String = "failure"
                                override val `val`: js.UndefOr[Type] = value
                            }
                        
                }
            

                trait Api {
                    type WitResult[+Ok, +Err] = Ok
                    object WitResult {
                        def ok[Ok](value: Ok): WitResult[Ok, Nothing] = value

                        def err[Err](value: Err): WitResult[Nothing, Err] = throw js.JavaScriptException(value)

                        val unit: WitResult[Unit, Nothing] = ()
                    }

                    type WitOption[+A] = js.UndefOr[A]
                    object WitOption {
                        def some[A](value: A): WitOption[A] = value

                        val none: WitOption[Nothing] = js.undefined

                        def fromOption[A](option: Option[A]) =
                        option match {
                            case Some(value) => value.asInstanceOf[js.UndefOr[A]]
                            case None        => js.undefined
                        }
                    }

                    type WitList[A] = js.Array[A]
                    object WitList {
                        def fromList[A](list: List[A]): WitList[A] = list.toJSArray
                    }
                    
                    def sendMessage(userId: Integer, message: String, channel: MessageChannel): MessageResult
def getMessageStatus(messageId: Integer): MessageStatus
def listUserMessages(userId: Integer): WitList[Message]
                }
            