
                // Generated by golem-scalajs-wit-bindgen
                package tests

                import scala.scalajs.js
                import scala.scalajs.js.JSConverters._

                

                

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
                    
                    def getRandomBytes(len: Long): WitList[Byte]
def getRandomU64(): Long
                }
            