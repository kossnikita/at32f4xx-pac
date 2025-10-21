#[doc = "Register `EPT3` reader"]
pub type R = crate::R<EPT3_SPEC>;
#[doc = "Register `EPT3` writer"]
pub type W = crate::W<EPT3_SPEC>;
#[doc = "Field `EPTADDR` reader - Endpoint address"]
pub type EPTADDR_R = crate::FieldReader;
#[doc = "Field `EPTADDR` writer - Endpoint address"]
pub type EPTADDR_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `TXSTS` reader - Tx status"]
pub type TXSTS_R = crate::FieldReader;
#[doc = "Field `TXSTS` writer - Tx status"]
pub type TXSTS_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `TXDTS` reader - Tx data toggle synchronization"]
pub type TXDTS_R = crate::BitReader;
#[doc = "Field `TXDTS` writer - Tx data toggle synchronization"]
pub type TXDTS_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TXTC` reader - Tx transaction completed"]
pub type TXTC_R = crate::BitReader;
#[doc = "Field `TXTC` writer - Tx transaction completed"]
pub type TXTC_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EXF` reader - Endpoint extend function"]
pub type EXF_R = crate::BitReader;
#[doc = "Field `EXF` writer - Endpoint extend function"]
pub type EXF_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TRANS_TYPE` reader - Transfer type"]
pub type TRANS_TYPE_R = crate::FieldReader;
#[doc = "Field `TRANS_TYPE` writer - Transfer type"]
pub type TRANS_TYPE_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `SETUPTC` reader - Setup transaction completed"]
pub type SETUPTC_R = crate::BitReader;
#[doc = "Field `SETUPTC` writer - Setup transaction completed"]
pub type SETUPTC_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RXSTS` reader - Rx Status"]
pub type RXSTS_R = crate::FieldReader;
#[doc = "Field `RXSTS` writer - Rx Status"]
pub type RXSTS_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `RXDTS` reader - Rx data toggle synchronization"]
pub type RXDTS_R = crate::BitReader;
#[doc = "Field `RXDTS` writer - Rx data toggle synchronization"]
pub type RXDTS_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RXTC` reader - Rx transaction completed"]
pub type RXTC_R = crate::BitReader;
#[doc = "Field `RXTC` writer - Rx transaction completed"]
pub type RXTC_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:3 - Endpoint address"]
    #[inline(always)]
    pub fn eptaddr(&self) -> EPTADDR_R {
        EPTADDR_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:5 - Tx status"]
    #[inline(always)]
    pub fn txsts(&self) -> TXSTS_R {
        TXSTS_R::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bit 6 - Tx data toggle synchronization"]
    #[inline(always)]
    pub fn txdts(&self) -> TXDTS_R {
        TXDTS_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Tx transaction completed"]
    #[inline(always)]
    pub fn txtc(&self) -> TXTC_R {
        TXTC_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Endpoint extend function"]
    #[inline(always)]
    pub fn exf(&self) -> EXF_R {
        EXF_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bits 9:10 - Transfer type"]
    #[inline(always)]
    pub fn trans_type(&self) -> TRANS_TYPE_R {
        TRANS_TYPE_R::new(((self.bits >> 9) & 3) as u8)
    }
    #[doc = "Bit 11 - Setup transaction completed"]
    #[inline(always)]
    pub fn setuptc(&self) -> SETUPTC_R {
        SETUPTC_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bits 12:13 - Rx Status"]
    #[inline(always)]
    pub fn rxsts(&self) -> RXSTS_R {
        RXSTS_R::new(((self.bits >> 12) & 3) as u8)
    }
    #[doc = "Bit 14 - Rx data toggle synchronization"]
    #[inline(always)]
    pub fn rxdts(&self) -> RXDTS_R {
        RXDTS_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Rx transaction completed"]
    #[inline(always)]
    pub fn rxtc(&self) -> RXTC_R {
        RXTC_R::new(((self.bits >> 15) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("EPT3")
            .field("eptaddr", &self.eptaddr())
            .field("txsts", &self.txsts())
            .field("txdts", &self.txdts())
            .field("txtc", &self.txtc())
            .field("exf", &self.exf())
            .field("trans_type", &self.trans_type())
            .field("setuptc", &self.setuptc())
            .field("rxsts", &self.rxsts())
            .field("rxdts", &self.rxdts())
            .field("rxtc", &self.rxtc())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:3 - Endpoint address"]
    #[inline(always)]
    pub fn eptaddr(&mut self) -> EPTADDR_W<'_, EPT3_SPEC> {
        EPTADDR_W::new(self, 0)
    }
    #[doc = "Bits 4:5 - Tx status"]
    #[inline(always)]
    pub fn txsts(&mut self) -> TXSTS_W<'_, EPT3_SPEC> {
        TXSTS_W::new(self, 4)
    }
    #[doc = "Bit 6 - Tx data toggle synchronization"]
    #[inline(always)]
    pub fn txdts(&mut self) -> TXDTS_W<'_, EPT3_SPEC> {
        TXDTS_W::new(self, 6)
    }
    #[doc = "Bit 7 - Tx transaction completed"]
    #[inline(always)]
    pub fn txtc(&mut self) -> TXTC_W<'_, EPT3_SPEC> {
        TXTC_W::new(self, 7)
    }
    #[doc = "Bit 8 - Endpoint extend function"]
    #[inline(always)]
    pub fn exf(&mut self) -> EXF_W<'_, EPT3_SPEC> {
        EXF_W::new(self, 8)
    }
    #[doc = "Bits 9:10 - Transfer type"]
    #[inline(always)]
    pub fn trans_type(&mut self) -> TRANS_TYPE_W<'_, EPT3_SPEC> {
        TRANS_TYPE_W::new(self, 9)
    }
    #[doc = "Bit 11 - Setup transaction completed"]
    #[inline(always)]
    pub fn setuptc(&mut self) -> SETUPTC_W<'_, EPT3_SPEC> {
        SETUPTC_W::new(self, 11)
    }
    #[doc = "Bits 12:13 - Rx Status"]
    #[inline(always)]
    pub fn rxsts(&mut self) -> RXSTS_W<'_, EPT3_SPEC> {
        RXSTS_W::new(self, 12)
    }
    #[doc = "Bit 14 - Rx data toggle synchronization"]
    #[inline(always)]
    pub fn rxdts(&mut self) -> RXDTS_W<'_, EPT3_SPEC> {
        RXDTS_W::new(self, 14)
    }
    #[doc = "Bit 15 - Rx transaction completed"]
    #[inline(always)]
    pub fn rxtc(&mut self) -> RXTC_W<'_, EPT3_SPEC> {
        RXTC_W::new(self, 15)
    }
}
#[doc = "endpoint 3 register\n\nYou can [`read`](crate::Reg::read) this register and get [`ept3::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ept3::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct EPT3_SPEC;
impl crate::RegisterSpec for EPT3_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ept3::R`](R) reader structure"]
impl crate::Readable for EPT3_SPEC {}
#[doc = "`write(|w| ..)` method takes [`ept3::W`](W) writer structure"]
impl crate::Writable for EPT3_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets EPT3 to value 0"]
impl crate::Resettable for EPT3_SPEC {}
