#[doc = "Register `EPT4` reader"]
pub type R = crate::R<EPT4_SPEC>;
#[doc = "Register `EPT4` writer"]
pub type W = crate::W<EPT4_SPEC>;
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
        f.debug_struct("EPT4")
            .field("eptaddr", &format_args!("{}", self.eptaddr().bits()))
            .field("txsts", &format_args!("{}", self.txsts().bits()))
            .field("txdts", &format_args!("{}", self.txdts().bit()))
            .field("txtc", &format_args!("{}", self.txtc().bit()))
            .field("exf", &format_args!("{}", self.exf().bit()))
            .field("trans_type", &format_args!("{}", self.trans_type().bits()))
            .field("setuptc", &format_args!("{}", self.setuptc().bit()))
            .field("rxsts", &format_args!("{}", self.rxsts().bits()))
            .field("rxdts", &format_args!("{}", self.rxdts().bit()))
            .field("rxtc", &format_args!("{}", self.rxtc().bit()))
            .finish()
    }
}
impl core::fmt::Debug for crate::generic::Reg<EPT4_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
impl W {
    #[doc = "Bits 0:3 - Endpoint address"]
    #[inline(always)]
    #[must_use]
    pub fn eptaddr(&mut self) -> EPTADDR_W<EPT4_SPEC> {
        EPTADDR_W::new(self, 0)
    }
    #[doc = "Bits 4:5 - Tx status"]
    #[inline(always)]
    #[must_use]
    pub fn txsts(&mut self) -> TXSTS_W<EPT4_SPEC> {
        TXSTS_W::new(self, 4)
    }
    #[doc = "Bit 6 - Tx data toggle synchronization"]
    #[inline(always)]
    #[must_use]
    pub fn txdts(&mut self) -> TXDTS_W<EPT4_SPEC> {
        TXDTS_W::new(self, 6)
    }
    #[doc = "Bit 7 - Tx transaction completed"]
    #[inline(always)]
    #[must_use]
    pub fn txtc(&mut self) -> TXTC_W<EPT4_SPEC> {
        TXTC_W::new(self, 7)
    }
    #[doc = "Bit 8 - Endpoint extend function"]
    #[inline(always)]
    #[must_use]
    pub fn exf(&mut self) -> EXF_W<EPT4_SPEC> {
        EXF_W::new(self, 8)
    }
    #[doc = "Bits 9:10 - Transfer type"]
    #[inline(always)]
    #[must_use]
    pub fn trans_type(&mut self) -> TRANS_TYPE_W<EPT4_SPEC> {
        TRANS_TYPE_W::new(self, 9)
    }
    #[doc = "Bit 11 - Setup transaction completed"]
    #[inline(always)]
    #[must_use]
    pub fn setuptc(&mut self) -> SETUPTC_W<EPT4_SPEC> {
        SETUPTC_W::new(self, 11)
    }
    #[doc = "Bits 12:13 - Rx Status"]
    #[inline(always)]
    #[must_use]
    pub fn rxsts(&mut self) -> RXSTS_W<EPT4_SPEC> {
        RXSTS_W::new(self, 12)
    }
    #[doc = "Bit 14 - Rx data toggle synchronization"]
    #[inline(always)]
    #[must_use]
    pub fn rxdts(&mut self) -> RXDTS_W<EPT4_SPEC> {
        RXDTS_W::new(self, 14)
    }
    #[doc = "Bit 15 - Rx transaction completed"]
    #[inline(always)]
    #[must_use]
    pub fn rxtc(&mut self) -> RXTC_W<EPT4_SPEC> {
        RXTC_W::new(self, 15)
    }
}
#[doc = "endpoint 4 register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ept4::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ept4::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct EPT4_SPEC;
impl crate::RegisterSpec for EPT4_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ept4::R`](R) reader structure"]
impl crate::Readable for EPT4_SPEC {}
#[doc = "`write(|w| ..)` method takes [`ept4::W`](W) writer structure"]
impl crate::Writable for EPT4_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets EPT4 to value 0"]
impl crate::Resettable for EPT4_SPEC {
    const RESET_VALUE: u32 = 0;
}
