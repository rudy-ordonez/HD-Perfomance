import static com.kms.katalon.core.checkpoint.CheckpointFactory.findCheckpoint
import static com.kms.katalon.core.testcase.TestCaseFactory.findTestCase
import static com.kms.katalon.core.testdata.TestDataFactory.findTestData
import static com.kms.katalon.core.testobject.ObjectRepository.findTestObject
import static com.kms.katalon.core.testobject.ObjectRepository.findWindowsObject
import com.kms.katalon.core.checkpoint.Checkpoint as Checkpoint
import com.kms.katalon.core.cucumber.keyword.CucumberBuiltinKeywords as CucumberKW
import com.kms.katalon.core.mobile.keyword.MobileBuiltInKeywords as Mobile
import com.kms.katalon.core.model.FailureHandling as FailureHandling
import com.kms.katalon.core.testcase.TestCase as TestCase
import com.kms.katalon.core.testdata.TestData as TestData
import com.kms.katalon.core.testobject.TestObject as TestObject
import com.kms.katalon.core.webservice.keyword.WSBuiltInKeywords as WS
import com.kms.katalon.core.webui.keyword.WebUiBuiltInKeywords as WebUI
import com.kms.katalon.core.windows.keyword.WindowsBuiltinKeywords as Windows
import internal.GlobalVariable as GlobalVariable
import org.openqa.selenium.Keys as Keys
import com.kms.katalon.core.testng.keyword.TestNGBuiltinKeywords as TestNGKW

f = new File('C://Temp/perf.csv')

WebUI.delay(5)

WebUI.click(findTestObject('Help Desk/Tickets Grid Perf/b_Tickets'))

WebUI.click(findTestObject('Help Desk/Tickets Grid Perf/a_All Tickets'))

WebUI.delay(5)

WebUI.click(findTestObject('Help Desk/Tickets Grid Perf/Single Ticket/a_16742'))

for (int i = 0; i < 26; i++) {
    long a1 = System.currentTimeMillis()

    WebUI.verifyElementClickable(findTestObject('Help Desk/Tickets Grid Perf/Single Ticket/svg_Open Print Preview'), FailureHandling.STOP_ON_FAILURE)

    long a2 = System.currentTimeMillis()

    (((f << new Date(a1).format('yyyy-MM-dd HH:mm:ss.SSS')) << ',') << ((a2 - a1) / 1000)) << '\r\n'

    WebUI.refresh()
}

WebUI.click(findTestObject('Help Desk/Logout/a_Rudy Ordonez'))

WebUI.click(findTestObject('null'))

WebUI.closeBrowser()

