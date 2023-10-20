#[doc = "Register `MACA3H` reader"]
pub type R = crate::R<MACA3H_SPEC>;
#[doc = "Register `MACA3H` writer"]
pub type W = crate::W<MACA3H_SPEC>;
#[doc = "Field `MA3H` reader - MAC address3 high"]
pub type MA3H_R = crate::FieldReader<u16>;
#[doc = "Field `MA3H` writer - MAC address3 high"]
pub type MA3H_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `MBC` reader - Mask byte control"]
pub type MBC_R = crate::FieldReader;
#[doc = "Field `MBC` writer - Mask byte control"]
pub type MBC_W<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "Field `SA` reader - Source address"]
pub type SA_R = crate::BitReader;
#[doc = "Field `SA` writer - Source address"]
pub type SA_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `AE` reader - Address enable"]
pub type AE_R = crate::BitReader;
#[doc = "Field `AE` writer - Address enable"]
pub type AE_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:15 - MAC address3 high"]
    #[inline(always)]
    pub fn ma3h(&self) -> MA3H_R {
        MA3H_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 24:29 - Mask byte control"]
    #[inline(always)]
    pub fn mbc(&self) -> MBC_R {
        MBC_R::new(((self.bits >> 24) & 0x3f) as u8)
    }
    #[doc = "Bit 30 - Source address"]
    #[inline(always)]
    pub fn sa(&self) -> SA_R {
        SA_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - Address enable"]
    #[inline(always)]
    pub fn ae(&self) -> AE_R {
        AE_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MACA3H")
            .field("ma3h", &format_args!("{}", self.ma3h().bits()))
            .field("mbc", &format_args!("{}", self.mbc().bits()))
            .field("sa", &format_args!("{}", self.sa().bit()))
            .field("ae", &format_args!("{}", self.ae().bit()))
            .finish()
    }
}
impl core::fmt::Debug for crate::generic::Reg<MACA3H_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bits 0:15 - MAC address3 high"]
    #[inline(always)]
    #[must_use]
    pub fn ma3h(&mut self) -> MA3H_W<MACA3H_SPEC> {
        MA3H_W::new(self, 0)
    }
    #[doc = "Bits 24:29 - Mask byte control"]
    #[inline(always)]
    #[must_use]
    pub fn mbc(&mut self) -> MBC_W<MACA3H_SPEC> {
        MBC_W::new(self, 24)
    }
    #[doc = "Bit 30 - Source address"]
    #[inline(always)]
    #[must_use]
    pub fn sa(&mut self) -> SA_W<MACA3H_SPEC> {
        SA_W::new(self, 30)
    }
    #[doc = "Bit 31 - Address enable"]
    #[inline(always)]
    #[must_use]
    pub fn ae(&mut self) -> AE_W<MACA3H_SPEC> {
        AE_W::new(self, 31)
    }
    #[doc = r" Writes raw bits to the register."]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = r" Passing incorrect value can cause undefined behaviour. See reference manual"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "Ethernet MAC address 3 high register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`maca3h::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`maca3h::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MACA3H_SPEC;
impl crate::RegisterSpec for MACA3H_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`maca3h::R`](R) reader structure"]
impl crate::Readable for MACA3H_SPEC {}
#[doc = "`write(|w| ..)` method takes [`maca3h::W`](W) writer structure"]
impl crate::Writable for MACA3H_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets MACA3H to value 0xffff"]
impl crate::Resettable for MACA3H_SPEC {
    const RESET_VALUE: Self::Ux = 0xffff;
}
